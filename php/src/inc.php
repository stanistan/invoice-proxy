<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US.UTF-8');

require_once __DIR__ . '/transforms.php';
require_once __DIR__ . '/Response.php';
require_once __DIR__ . '/Pipeline.php';
require_once __DIR__ . '/DiskCache.php';
require_once __DIR__ . '/FetchCtx.php';
require_once __DIR__ . '/pipelines.php';

//
// main handler for the request/response structure
function route(string $route) : Response {

    $ctx = new FetchCtx($_ENV['AIRTABLE_KEY'], $_ENV['AIRTABLE_APP'], isset($_GET['r']));

    try {
        // extremely bare-bones router
        $pieces = array_values(array_filter(explode('/', $route)));
        switch (count($pieces)) {
        case 2:
            [ $first, $second ] = $pieces;
            switch ($first) {
            case 'invoice':
                $invoice = pipelines\invoice($ctx);
                return new Response(200, $invoice($second));
            case 'invoice-by-id':
                $invoice = pipelines\invoiceById($ctx);
                return new Response(200, $invoice($second));
            }
        }
    } catch (ResponseError $e) {
        return $e->asResponse();
    }

    // if nothing matched, we send back a 404
    return new Response(404, [ 'error' => 'Invalid route: ' . $route ]);
}

