<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US');

require_once __DIR__ . '/inc.php';

try {
    $ctx = new Pipelines(new FetchCtx($_ENV['AIRTABLE_KEY'], $_ENV['AIRTABLE_APP']));
    $re = handle($ctx, $_SERVER['PHP_SELF'])->send();
} catch (Throwable $e) {
    $re = new Response(500, [
        'error' => $e->getMessage(),
    ]);
    $re->send();
}
