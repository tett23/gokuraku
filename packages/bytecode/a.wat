(module
    (func $main (export "main")
        (call $add (i64.const 1) (i64.const 2))
        drop
    )
    (func $add (export "add") (param $a i64) (param $b i64) (result i64)
        (local.get $a)
        (local.get $b)
        (i64.add)
    )
    (start $main)
)
