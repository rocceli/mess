(module
    ;; params: data_ptr, data_len_ptr
    (import "seal" "seal input" (func $seal_input (param i32 i32)))
    (import "env" "memory" (memory 1 1))

    (func (export "deploy")
        ;; execute some code on contract deployment ( we do nothing here )
    )

    (func (export "call")
        ;;execute some code on contract call
        (i32.store (i32.const 4) (i32.const 256)) ;; store thr length of our buffer at mem[4]
        (call $seal_input (i32.const 0) (i32.const 256)) ;; call seal_input with buffer at mem[0] and length at mem[4]
    )
)