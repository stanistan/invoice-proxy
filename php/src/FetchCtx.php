<?php declare(strict_types=1);

use transforms as t;

class FetchCtx {

    private string $base_url;
    private array $stream_opts;
    private array $cache = [];
    private DiskCache $disk_cache;

    public function __construct(
        string $auth_key,
        string $space,
        bool $refresh_disk_cache = false
    ) {

        if (!$auth_key || !$space) {
            throw new Exception("Cannot create a request context");
        }

        $this->disk_cache = new DiskCache("/tmp/invoice-proxy-cache/", $refresh_disk_cache);
        $this->base_url = "https://api.airtable.com/v0/{$space}/";

        $this->stream_opts = [
            'http' => [
                'method' => 'GET',
                'header' => "Authorization: Bearer $auth_key"
            ]
        ];
    }

    private function trap(string $message) {
        return function(Throwable $e) use($message) {
            if ($e instanceof t\InvalidTransform) {
                throw new ResponseError(404, $message, $e);
            } else {
                throw $e;
            }
        };
    }

    private function req(string $path) : array {
        //
        // first we check to see if this response is
        // in memory, return it from there, this is local
        // to each HTTP request/process, so either way if we
        // do a `refresh_disk_cache` or not, this is going
        // to be _fresh_ for that individual proxy request.
        if (isset($this->cache[$path])) {
            return $this->cache[$path];
        }

        return $this->cache[$path] = $this->disk_cache->getOrSetWith(
            $path,
            function() use($path) {
                $url = $this->base_url . $path;
                $context = stream_context_create($this->stream_opts);
                $json = @file_get_contents($url, false, $context);
                if (!$json) {
                    $e = error_get_last();
                    throw new ResponseError(404, "Failed to fetch path=$path... {$e['message']}");
                }
                return json_decode($json, true);
            }
        );
    }

    private function queryRequestFor(string $table, string $field_name) : callable {
        return pipeline(
            function(string $value) use($table, $field_name) {
                $path = rawurlencode($table) . '?' . http_build_query([
                    'filterByFormula' => "{$field_name} = '{$value}'"
                ]);
                return $this->req($path);
            },
            t\enter('records'),
            t\first(),
            t\fields()
        )->withTrap($this->trap("Querying for data in $table by $field_name returned nothing"));
    }

    private function idRequestFor(string $table) : callable {
        return pipeline(
            function(string $id) use ($table) {
                return $this->req(rawurlencode($table) . '/' . $id);
            },
            t\fields()
        )->withTrap($this->trap("idRequestFor for $table returned nothing"));
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

    public function invoiceById() {
        return $this->queryRequestFor('Invoice', 'ID');
    }

}
