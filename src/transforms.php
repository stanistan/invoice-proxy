<?php declare(strict_types=1);

namespace transforms;

function money() : callable {
    return function($num) : string {
        $number = $num ?: 0.00;
        return \money_format('%.2n', $number);
    };
}

function first($fn = null) : callable {
    return function($list) use($fn) {
        $first = \reset($list);
        return $fn ? $fn($first) : $first;
    };
}

function map(callable $fn) {
    return function($ids) use($fn) {
        return \array_map($fn, $ids);
    };
}

function fields() {
    return function($object) {
        return $object['fields'];
    };
}

function discard(...$keys) {
    return function($object) use($keys) {
        foreach ($keys as $k) {
            unset($object[$k]);
        }
        return $object;
    };
}

function reduce(array $fns) {
    return function($object) use($fns) {
        foreach ($fns as $fn) {
            $object = $fn($object);
        }
        return $object;
    };
}

function mapKeys(...$field_transforms) {
    return function($object) use($field_transforms) {
        foreach ($field_transforms as $field_transform) {
            $field_name = array_shift($field_transform);
            $object[$field_name] = reduce($field_transform)($object[$field_name]);
        }
        return $object;
    };
}

function pickKeys(...$keys) {
    return function($object) use($keys) {
        $output = [];
        foreach ($keys as $k) {
            $output[$k] = $object[$k];
        }
        return $output;
    };
}

function mapAndPickKeys(...$field_transforms) {
    $keys = map(first())($field_transforms);
    return reduce([mapKeys(...$field_transforms), pickKeys(...$keys)]);
}
