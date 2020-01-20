<?php declare(strict_types=1);

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

class ResponseError extends Exception {

    private int $http_code;

    public function __construct($code, $message, Throwable $e) {
        $this->http_code = $code;
        parent::__construct($message, 0, $e);
    }

    public function asResponse() : Response {
        return new Response($this->http_code, [
            'error' => 'Not found... ' . $this->getMessage()
        ]);
    }
}

