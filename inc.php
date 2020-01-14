<?php declare(strict_types=1);

// main handler for things
function handle(Pipelines $pipelines, string $route) : Response {
    $pieces = array_values(array_filter(explode('/', $route)));
    switch (count($pieces)) {
    case 2:
        [ $first, $second ] = $pieces;
        switch ($first) {
        case 'invoice':
            $pipeline = $pipelines->invoice();
            return new Response(200, $pipeline($second));
            break;
        }
    }

    return new Response(404, [ 'error' => 'Invalid route: ' . $route ]);
}

class Response {

    private int $code;
    private array $body;

    public function __construct(int $code, array $body) {
        $this->code = $code;
        $this->body = $body;
    }

    public function send() {
        header('Content-Type: application/json');
        http_response_code($this->code);
        echo json_encode($this->body, JSON_PRETTY_PRINT);
    }
}


function money() : callable {
    return static function($num) : string {
        $num = $num ?: 0;
        return money_format('%n', $num);
    };
}

function first($fn = null) : callable {
    return function($list) use($fn) {
        $first = reset($list);
        return $fn ? $fn($first) : $first;
    };
}

function map(callable $fn) {
    return function($ids) use($fn) {
        return array_map($fn, $ids);
    };
}

function mapField($array, $name, $fns) {
    $value = $array['fields'][$name];
    foreach ($fns as $fn) {
        $value = $fn($value);
    }

    $array['fields'][$name] = $value;
    return $array;
}

function pipeline($function, ...$pairs) : callable {
    return function(...$args) use($function, $pairs) : array {
        $object = $function(...$args);
        foreach ($pairs as $args) {
            $field = array_shift($args);
            $object = mapField($object, $field, $args);
        }
        return $object;
    };
}

class FetchCtx {

    private $base_url;
    private $stream_opts;
    private $cache = [];

    private const CACHE_VERSION = 'v1';

    public function __construct(string $auth_key, string $space) {

        if (!$auth_key || !$space) {
            throw new Exception("Cannot create a request context");
        }

        $this->base_url = "https://api.airtable.com/v0/{$space}/";
        $this->stream_opts = [
            'http' => [
                'method' => 'GET',
                'header' => "Authorization: Bearer $auth_key"
            ]
        ];
    }

    private function req(string $path) : array {

        // first we check to see if this response is
        // in memory, return it from there,
        if (isset($this->cache[$path])) {
            return $this->cache[$path];
        }

        $disk_cache_dir = "/tmp/invoice-proxy-cache/" . self::CACHE_VERSION . "/";
        $disk_cache_path = $disk_cache_dir . md5($path);

        if (!file_exists($disk_cache_path)) {
            $url = $this->base_url . $path;
            $context = stream_context_create($this->stream_opts);
            $content = @file_get_contents($url, false, $context);
            if (!$content) {
                throw new Exception("Could not fetch content for url: $url");
            }
            if (!is_dir($disk_cache_dir)) {
                mkdir($disk_cache_dir, 0777, true);
            }
            file_put_contents($disk_cache_path, $content);
        } else {
            $content = file_get_contents($disk_cache_path);
        }

        return $this->cache[$path] = json_decode($content, true);
    }

    private function idRequestFor(string $table) : callable {
        return function(string $id) use ($table) {
            return $this->req(rawurlencode($table) . '/' . $id);
        };
    }

    public function invoice() {
        return $this->idRequestFor('Invoice');
    }

    public function me() {
        return $this->idRequestFor('Me');
    }

    public function client() {
        return $this->idRequestFor('Clients');
    }

    public function invoiceItem() {
        return $this->idRequestFor('Invoice Item');
    }

    public function unit() {
        return $this->idRequestFor('Invoice Units');
    }

    public function invoiceRate() {
        return $this->idRequestFor('Invoice Rates');
    }

}

class Pipelines {

    private FetchCtx $fetch;

    public function __construct(FetchCtx $req_ctx) {
        $this->fetch = $req_ctx;
    }

    public function invoiceItem() {
        return pipeline($this->fetch->invoiceItem(),
            [ 'Amount', money() ],
            [ 'Invoice Rate', first(), $this->fetch->invoiceRate() ],
        );
    }

    public function from() {
        return pipeline($this->fetch->me(),
            [ 'Address', fn($address) => explode("\n", $address) ],
        );
    }

    public function invoice() {
        return pipeline($this->fetch->invoice(),
            [ 'Total Amount', money() ],
            [ 'From', first(), $this->from() ],
            [ 'Client', first(), $this->fetch->client() ],
            [ 'Invoice Item', map($this->invoiceItem()) ],
        );
    }

}
