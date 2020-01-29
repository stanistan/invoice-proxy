<?php declare(strict_types=1);

class DiskCache {

    private const CACHE_VERSION = 'v2';

    private string $dir;
    private bool $write_only;

    public function __construct(string $dir, bool $write_only) {
        $this->dir = $dir . self::CACHE_VERSION . '/';
        $this->write_only = $write_only;
    }

    private function filePath(string $key) : string {
        return $this->dir . md5($key);
    }

    public function has(string $key) : bool {
        return !$this->write_only && file_exists($this->filePath($key));
    }

    public function getOrSetWith(string $key, $fn) {
        if (!$this->has($key)) {
            $stuff = $fn();
            $this->set($key, $stuff);
            return $stuff;
        } else {
            return $this->get($key);
        }
    }

    public function get(string $key) : array {
        if ($this->write_only) {
            throw new Exception("Cannot read from write-only cache");
        }

        $cache_contents = json_decode(file_get_contents($this->filePath($key)), true);
        return $cache_contents['content'];
    }

    public function set(string $key, $value) {
        $content = [
            'version' => self::CACHE_VERSION,
            'key' => $key,
            'content' => $value
        ];
        file_put_contents($this->filePath($key), json_encode($content));
    }

}
