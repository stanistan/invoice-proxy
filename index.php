<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US.UTF-8');

require_once __DIR__ . '/inc.php';

try {

    $ctx = new Pipelines(new FetchCtx(
        $_ENV['AIRTABLE_KEY'],
        $_ENV['AIRTABLE_APP']
    ));

    handle($ctx, $_SERVER['PHP_SELF'])->send();

} catch (Throwable $e) {
    (new Response(500, [ 'error' => $e->getMessage(), ]))->send();
}
