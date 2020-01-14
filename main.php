<?php declare(strict_types=1);

setlocale(LC_MONETARY, 'en_US.UTF-8');

require_once __DIR__ . '/src/inc.php';

$ctx = new Pipelines(new FetchCtx(
    $_ENV['AIRTABLE_KEY'],
    $_ENV['AIRTABLE_APP']
));

handle($ctx, $_SERVER['PHP_SELF'])->send();

