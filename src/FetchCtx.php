<?php declare(strict_types=1);

use transforms as t;

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
        return pipeline(
            function(string $id) use ($table) {
                return $this->req(rawurlencode($table) . '/' . $id);
            },
            t\fields()
        );
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
