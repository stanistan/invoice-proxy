<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US.UTF-8');

require_once __DIR__ . '/src/inc.php';

$ctx = new Pipelines(new FetchCtx(
    $_ENV['AIRTABLE_KEY'],
    $_ENV['AIRTABLE_APP']
));

// main handler for the request/response structure
function route(Pipelines $pipelines, string $route) : Response {

    // extremely bare-bones router
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

    // if nothing matched, we send back a 404
    return new Response(404, [ 'error' => 'Invalid route: ' . $route ]);
}

route($ctx, $_SERVER['PHP_SELF'])->send();
