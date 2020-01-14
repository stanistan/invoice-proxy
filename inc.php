<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US');

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

// main handler for things
function handle(Ctx $ctx, string $route) : Response {
    $pieces = array_values(array_filter(explode('/', $route)));
    if ($pieces[0] == 'invoice' && count($pieces) === 2) {
        $invoice_id = $pieces[1];
        $invoice = $ctx->invoice($invoice_id);
        return new Response(200, $invoice);
    }

    return new Response(404, [ 'error' => 'Invalid route: ' . $route ]);
}

function money($num) : string {
    $num = $num ?: 0;
    return money_format('%n', $num);
}

function asMethod($ob, $name) {
    return Closure::fromCallable([ $ob, $name ]);
}

function asFn($name) {
    return Closure::fromCallable($name);
}

function mapField($array, $name, $fns) {

    $value = $array['fields'][$name];
    foreach ($fns as $fn) {
        $value = $fn($value);
    }

    $array['fields'][$name] = $value;
    return $array;
}

function pipeline($object, ...$pairs) {
    foreach ($pairs as $args) {
        $field = array_shift($args);
        $object = mapField($object, $field, $args);
    }
    return $object;
}

class RequestCtx {

    private $base_url;
    private $stream_opts;
    private $cache = [];

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

    private function reqById(string $table, string $id) {
        return $this->req(rawurlencode($table) . '/' . $id);
    }

    private function req(string $path) : array {
        if (isset($this->cache[$path])) {
            return $this->cache[$path];
        }
        $context = stream_context_create($this->stream_opts);
        $url = $this->base_url . $path;
        $content = @file_get_contents($url, false, $context);
        if (!$content) {
            throw new Exception("Could not fetch content for url: $url");
        }
        return $this->cache[$path] = json_decode($content, true);
    }

    public function invoice($id) {
        return $this->reqById('Invoice', $id);
    }

    public function client($id) {
        return $this->reqById('Clients', $id);
    }

    public function invoiceItem($id) {
        return $this->reqById('Invoice Item', $id);
    }

    public function unit($id) {
        return $this->reqById('Invoice Units', $id);
    }

    public function invoiceRate($id) {
        return $this->reqById('Invoice Rates', $id);
    }

}

class Ctx {

    private $req_ctx;

    public function __construct(RequestCtx $req_ctx) {
        $this->req_ctx = $req_ctx;
    }

    public function invoiceItem($id) {
        return pipeline($this->req_ctx->invoiceItem($id),
            [ 'Amount', asFn('money') ],
            [ 'Invoice Rate', asFn('reset'), asMethod($this->req_ctx, 'invoiceRate') ],
        );
    }

    public function invoiceItems($ids) {
        return array_map(asMethod($this, 'invoiceItem'), $ids);
    }

    public function invoice($id) {
        return pipeline($this->req_ctx->invoice($id),
            [ 'Total Amount', asFn('money') ],
            [ 'Client', asFn('reset'), asMethod($this->req_ctx, 'client') ],
            [ 'Invoice Item', asMethod($this, 'invoiceItems') ]
        );
    }

}
