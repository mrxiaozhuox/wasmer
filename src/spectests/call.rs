// Rust test file autogenerated from call.wast
// Please do not modify

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use wabt::wat2wasm;


// Line 3
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (result i64)))
      (type (;2;) (func (result f32)))
      (type (;3;) (func (result f64)))
      (type (;4;) (func (param i32) (result i32)))
      (type (;5;) (func (param i64) (result i64)))
      (type (;6;) (func (param f32) (result f32)))
      (type (;7;) (func (param f64) (result f64)))
      (type (;8;) (func (param f32 i32) (result i32)))
      (type (;9;) (func (param i32 i64) (result i64)))
      (type (;10;) (func (param f64 f32) (result f32)))
      (type (;11;) (func (param i64 f64) (result f64)))
      (type (;12;) (func (param i64 i64) (result i64)))
      (type (;13;) (func (param i64) (result i32)))
      (type (;14;) (func))
      (func (;0;) (type 0) (result i32)
        i32.const 306)
      (func (;1;) (type 1) (result i64)
        i64.const 356)
      (func (;2;) (type 2) (result f32)
        f32.const 0x1.e64p+11 (;=3890;))
      (func (;3;) (type 3) (result f64)
        f64.const 0x1.ec8p+11 (;=3940;))
      (func (;4;) (type 4) (param i32) (result i32)
        get_local 0)
      (func (;5;) (type 5) (param i64) (result i64)
        get_local 0)
      (func (;6;) (type 6) (param f32) (result f32)
        get_local 0)
      (func (;7;) (type 7) (param f64) (result f64)
        get_local 0)
      (func (;8;) (type 8) (param f32 i32) (result i32)
        get_local 1)
      (func (;9;) (type 9) (param i32 i64) (result i64)
        get_local 1)
      (func (;10;) (type 10) (param f64 f32) (result f32)
        get_local 1)
      (func (;11;) (type 11) (param i64 f64) (result f64)
        get_local 1)
      (func (;12;) (type 0) (result i32)
        call 0)
      (func (;13;) (type 1) (result i64)
        call 1)
      (func (;14;) (type 2) (result f32)
        call 2)
      (func (;15;) (type 3) (result f64)
        call 3)
      (func (;16;) (type 0) (result i32)
        i32.const 32
        call 4)
      (func (;17;) (type 1) (result i64)
        i64.const 64
        call 5)
      (func (;18;) (type 2) (result f32)
        f32.const 0x1.51eb86p+0 (;=1.32;)
        call 6)
      (func (;19;) (type 3) (result f64)
        f64.const 0x1.a3d70a3d70a3dp+0 (;=1.64;)
        call 7)
      (func (;20;) (type 0) (result i32)
        f32.const 0x1.00ccccp+5 (;=32.1;)
        i32.const 32
        call 8)
      (func (;21;) (type 1) (result i64)
        i32.const 32
        i64.const 64
        call 9)
      (func (;22;) (type 2) (result f32)
        f64.const 0x1p+6 (;=64;)
        f32.const 0x1p+5 (;=32;)
        call 10)
      (func (;23;) (type 3) (result f64)
        i64.const 64
        f64.const 0x1.0066666666666p+6 (;=64.1;)
        call 11)
      (func (;24;) (type 5) (param i64) (result i64)
        get_local 0
        i64.eqz
        if (result i64)  ;; label = @1
          i64.const 1
        else
          get_local 0
          get_local 0
          i64.const 1
          i64.sub
          call 24
          i64.mul
        end)
      (func (;25;) (type 12) (param i64 i64) (result i64)
        get_local 0
        i64.eqz
        if (result i64)  ;; label = @1
          get_local 1
        else
          get_local 0
          i64.const 1
          i64.sub
          get_local 0
          get_local 1
          i64.mul
          call 25
        end)
      (func (;26;) (type 5) (param i64) (result i64)
        get_local 0
        i64.const 1
        i64.le_u
        if (result i64)  ;; label = @1
          i64.const 1
        else
          get_local 0
          i64.const 2
          i64.sub
          call 26
          get_local 0
          i64.const 1
          i64.sub
          call 26
          i64.add
        end)
      (func (;27;) (type 13) (param i64) (result i32)
        get_local 0
        i64.eqz
        if (result i32)  ;; label = @1
          i32.const 44
        else
          get_local 0
          i64.const 1
          i64.sub
          call 28
        end)
      (func (;28;) (type 13) (param i64) (result i32)
        get_local 0
        i64.eqz
        if (result i32)  ;; label = @1
          i32.const 99
        else
          get_local 0
          i64.const 1
          i64.sub
          call 27
        end)
      (func (;29;) (type 14)
        call 29)
      (func (;30;) (type 14)
        call 31)
      (func (;31;) (type 14)
        call 30)
      (export \"type-i32\" (func 12))
      (export \"type-i64\" (func 13))
      (export \"type-f32\" (func 14))
      (export \"type-f64\" (func 15))
      (export \"type-first-i32\" (func 16))
      (export \"type-first-i64\" (func 17))
      (export \"type-first-f32\" (func 18))
      (export \"type-first-f64\" (func 19))
      (export \"type-second-i32\" (func 20))
      (export \"type-second-i64\" (func 21))
      (export \"type-second-f32\" (func 22))
      (export \"type-second-f64\" (func 23))
      (export \"fac\" (func 24))
      (export \"fac-acc\" (func 25))
      (export \"fib\" (func 26))
      (export \"even\" (func 27))
      (export \"odd\" (func 28))
      (export \"runaway\" (func 29))
      (export \"mutual-runaway\" (func 30)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, ImportObject::new()).expect("WASM can't be instantiated")
}

// Line 111
#[test]
fn l111_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 306 as i32);
}

// Line 112
#[test]
fn l112_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 356 as i64);
}

// Line 113
#[test]
fn l113_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 3890.0 as f32);
}

// Line 114
#[test]
fn l114_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 3940.0 as f64);
}

// Line 116
#[test]
fn l116_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-first-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 32 as i32);
}

// Line 117
#[test]
fn l117_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-first-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 64 as i64);
}

// Line 118
#[test]
fn l118_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-first-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1.32 as f32);
}

// Line 119
#[test]
fn l119_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-first-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1.64 as f64);
}

// Line 121
#[test]
fn l121_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-second-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 32 as i32);
}

// Line 122
#[test]
fn l122_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-second-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 64 as i64);
}

// Line 123
#[test]
fn l123_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-second-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 32.0 as f32);
}

// Line 124
#[test]
fn l124_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("type-second-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 64.1 as f64);
}

// Line 126
#[test]
fn l126_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(0 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 127
#[test]
fn l127_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(1 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 128
#[test]
fn l128_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(5 as i64, &vm_context);
    assert_eq!(result, 120 as i64);
}

// Line 129
#[test]
fn l129_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(25 as i64, &vm_context);
    assert_eq!(result, 7034535277573963776 as i64);
}

// Line 130
#[test]
fn l130_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac-acc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(0 as i64, 1 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 131
#[test]
fn l131_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac-acc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(1 as i64, 1 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 132
#[test]
fn l132_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac-acc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(5 as i64, 1 as i64, &vm_context);
    assert_eq!(result, 120 as i64);
}

// Line 134
#[test]
fn l134_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fac-acc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(25 as i64, 1 as i64, &vm_context);
    assert_eq!(result, 7034535277573963776 as i64);
}

// Line 138
#[test]
fn l138_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fib") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(0 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 139
#[test]
fn l139_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fib") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(1 as i64, &vm_context);
    assert_eq!(result, 1 as i64);
}

// Line 140
#[test]
fn l140_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fib") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(2 as i64, &vm_context);
    assert_eq!(result, 2 as i64);
}

// Line 141
#[test]
fn l141_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fib") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(5 as i64, &vm_context);
    assert_eq!(result, 8 as i64);
}

// Line 142
#[test]
fn l142_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("fib") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(20 as i64, &vm_context);
    assert_eq!(result, 10946 as i64);
}

// Line 144
#[test]
fn l144_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("even") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(0 as i64, &vm_context);
    assert_eq!(result, 44 as i32);
}

// Line 145
#[test]
fn l145_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("even") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(1 as i64, &vm_context);
    assert_eq!(result, 99 as i32);
}

// Line 146
#[test]
fn l146_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("even") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(100 as i64, &vm_context);
    assert_eq!(result, 44 as i32);
}

// Line 147
#[test]
fn l147_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("even") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(77 as i64, &vm_context);
    assert_eq!(result, 99 as i32);
}

// Line 148
#[test]
fn l148_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("odd") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(0 as i64, &vm_context);
    assert_eq!(result, 99 as i32);
}

// Line 149
#[test]
fn l149_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("odd") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(1 as i64, &vm_context);
    assert_eq!(result, 44 as i32);
}

// Line 150
#[test]
fn l150_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("odd") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(200 as i64, &vm_context);
    assert_eq!(result, 99 as i32);
}

// Line 151
#[test]
fn l151_assert_return_invoke() {
    let ResultObject { mut instance, module } = create_module_1();
    let func_index = match module.info.exports.get("odd") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i32 = get_instance_function!(instance, func_index);
    let vm_context = instance.generate_context();
    let result = invoke_fn(77 as i64, &vm_context);
    assert_eq!(result, 44 as i32);
}

// Line 153

// Line 154

// Line 160

#[test]
fn l160_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 10, 2, 5, 0, 16, 1, 69, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 167

#[test]
fn l167_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 8, 2, 96, 0, 0, 96, 0, 1, 126, 3, 3, 2, 0, 1, 10, 12, 2, 5, 0, 16, 1, 69, 11, 4, 0, 66, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 175

#[test]
fn l175_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 8, 2, 96, 0, 0, 96, 1, 127, 0, 3, 3, 2, 0, 1, 10, 9, 2, 4, 0, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 182

#[test]
fn l182_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 124, 127, 0, 3, 3, 2, 0, 1, 10, 9, 2, 4, 0, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 189

#[test]
fn l189_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 11, 2, 6, 0, 65, 1, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 196

#[test]
fn l196_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 20, 2, 15, 0, 68, 0, 0, 0, 0, 0, 0, 0, 64, 65, 1, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 204

#[test]
fn l204_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 127, 0, 3, 3, 2, 0, 1, 10, 12, 2, 7, 0, 1, 65, 1, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 211

#[test]
fn l211_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 127, 0, 3, 3, 2, 0, 1, 10, 12, 2, 7, 0, 65, 1, 1, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 218

#[test]
fn l218_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 124, 0, 3, 3, 2, 0, 1, 10, 20, 2, 15, 0, 68, 0, 0, 0, 0, 0, 0, 240, 63, 65, 1, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 225

#[test]
fn l225_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 124, 127, 0, 3, 3, 2, 0, 1, 10, 20, 2, 15, 0, 65, 1, 68, 0, 0, 0, 0, 0, 0, 240, 63, 16, 1, 11, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 236

#[test]
fn l236_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 16, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 240

#[test]
fn l240_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 0, 16, 148, 152, 219, 226, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}
