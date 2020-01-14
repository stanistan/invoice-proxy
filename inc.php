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
    return function($num) : string {
        $number = $num ?: 0.00;
        return money_format('%.2n', $number);
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

function fields() {
    return function($object) {
        return $object['fields'];
    };
}

function discard(...$keys) {
    return function($object) use($keys) {
        foreach ($keys as $k) {
            unset($object[$k]);
        }
        return $object;
    };
}

function reduce(array $fns) {
    return function($object) use($fns) {
        foreach ($fns as $fn) {
            $object = $fn($object);
        }
        return $object;
    };
}

function mapKeys(...$field_transforms) {
    return function($object) use($field_transforms) {
        foreach ($field_transforms as $field_transform) {
            $field_name = array_shift($field_transform);
            $object[$field_name] = reduce($field_transform)($object[$field_name]);
        }
        return $object;
    };
}

function pickKeys(...$keys) {
    return function($object) use($keys) {
        $output = [];
        foreach ($keys as $k) {
            $output[$k] = $object[$k];
        }
        return $output;
    };
}

function mapAndPickKeys(...$field_transforms) {
    $keys = map(first())($field_transforms);
    return reduce([mapKeys(...$field_transforms), pickKeys(...$keys)]);
}


class Pipeline {

    private $function;
    private $transforms;

    public function __construct(callable $function, $transforms) {
        $this->function = $function;
        $this->transforms = $transforms;
    }

    //
    // When the returned function is called, we apply the args to the
    // initial `$function`, this allows us to have lazy evaluation of
    // pipelines.
    public function __invoke(...$args) {
        $object = call_user_func_array($this->function, $args);
        $object = reduce($this->transforms)($object);
        return $object;
    }

}

function pipeline(callable $function, ...$field_transforms) : callable {
    return new Pipeline($function, $field_transforms);
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
        return pipeline(
            $this->fetch->invoiceItem(),
            fields(),
            mapAndPickKeys(
                [ 'Date' ],
                [ 'Description' ],
                [ 'Quantity' ],
                [ 'Amount', money() ],
                [ 'Invoice Rate', first(), $this->fetch->invoiceRate(), fields(), pickKeys('Name', 'Notes') ],
            ),
        );
    }

    public function from() {
        return pipeline(
            $this->fetch->me(),
            fields(),
            mapAndPickKeys(
                [ 'Name' ],
                [ 'Email' ],
                [ 'Address', fn($address) => explode("\n", $address) ]
            )
        );
    }

    public function client() {
        return pipeline(
            $this->fetch->client(),
            fields(),
            pickKeys('ContactName', 'Company', 'Website', 'ContactEmail')
        );
    }

    public function invoice() {
        return pipeline(
            $this->fetch->invoice(),
            fields(),
            mapAndPickKeys(
                [ 'ID' ],
                [ 'Date' ],
                [ 'Due Date' ],
                [ 'Invoice Number' ],
                [ 'Total Amount', money() ],
                [ 'From', first(), $this->from() ],
                [ 'Client', first(), $this->client() ],
                [ 'Invoice Item', map($this->invoiceItem()) ],
            )
        );
    }

}
