<?php declare(strict_types=1);
//
//
//
// This should be run as the main entrypoint for some server.
//
//
//
require_once __DIR__ . '/src/inc.php';
route($_SERVER['PHP_SELF'])->send();
