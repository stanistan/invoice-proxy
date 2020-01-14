<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US');

require_once __DIR__ . '/inc.php';

$ctx = new RequestCtx($_ENV['AIRTABLE_KEY'], $_ENV['AIRTABLE_APP']);
$ctx = new Ctx($ctx);

try {
    $re = handle($ctx, $_SERVER['PHP_SELF'])->send();
} catch (Throwable $e) {
    $re = new Response(500, [
        'error' => $e->getMessage(),
        'trace' => $e->getTraceAsString()
    ]);
    $re->send();
}
