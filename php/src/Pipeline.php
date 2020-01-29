<?php declare(strict_types=1);

use transforms as t;

class Pipeline {

    private $function;
    private $transforms;
    private $trap;

    public function __construct(callable $function, $transforms) {
        $this->function = $function;
        $this->transforms = $transforms;
    }

    public function withTrap(callable $function) {
        $this->trap = $function;
        return $this;
    }

    //
    // When the returned function is called, we apply the args to the
    // initial `$function`, this allows us to have lazy evaluation of
    // pipelines.
    public function __invoke(...$args) {
        try {
            $object = call_user_func_array($this->function, $args);
            $object = t\reduce($this->transforms)($object);
            return $object;
        } catch (Throwable $e) {
            if (!$this->trap) {
                throw $e;
            } else {
                return call_user_func($this->trap, $e);
            }
        }
    }
}

function pipeline(callable $function, ...$field_transforms) : Pipeline {
    return new Pipeline($function, $field_transforms);
}
