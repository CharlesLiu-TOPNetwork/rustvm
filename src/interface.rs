use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;

use crate::wasm_backend::compile;

#[no_mangle]
pub extern "C" fn validator_wasm_with_path(s: *const c_char) {
    let path;
    unsafe {
        path = CStr::from_ptr(s).to_str().unwrap();
        println!("here is wasm file path: {:?}", path);
    }
    let mut file = File::open(path).unwrap();

    // Read wasm
    let mut wasm = Vec::<u8>::new();
    file.read_to_end(&mut wasm).unwrap();

    println!("wasm: {:?}", wasm);

    let _module = compile(&wasm, None).unwrap();
    println!("contract compile check pass!");
}

#[no_mangle]
pub extern "C" fn validator_wasm_with_content(s: *const u8, size: i32) {
    let ptr = s;
    let mut wasm = Vec::<u8>::new();

    for i in 0..size as usize {
        unsafe {
            let iter = ((ptr as usize) + i) as *const u8;
            wasm.push(*iter);
            // println!("{}:{:?} : {:?}", i, iter, *iter);
        }
    }
    println!("wasm: {:?}", wasm);

    let _module = compile(&wasm, None).unwrap();
    println!("contract compile check pass!");
}

// todo delete below all test-only code.
// #[no_mangle]
// pub extern "C" fn test_str(s: *const c_char) {
//     println!("here is {:?}", s);
//     unsafe {
//         let ss = CStr::from_ptr(s);
//         println!("here is {:?}", ss);
//     }
//     println!("here is {:?}", CStr::from());
// }

#[no_mangle]
pub extern "C" fn test_wasm_file_path(s: *const c_char) {
    let path;
    unsafe {
        path = CStr::from_ptr(s).to_str().unwrap();
        println!("here is wasm file path: {:?}", path);
    }
    let mut file = File::open(path).unwrap();

    // Read wasm
    let mut wasm = Vec::<u8>::new();
    file.read_to_end(&mut wasm).unwrap();

    println!("wasm: {:?}", wasm);

    compile_test(&wasm).unwrap();
    println!("[test]contract compile check pass!");
}

#[no_mangle]
pub extern "C" fn test_wasm_file_content(s: *const u8, size: i32) {
    println!("size: {}", size);

    let ptr = s;
    let mut wasm = Vec::<u8>::new();

    for i in 0..size as usize {
        unsafe {
            let iter = ((ptr as usize) + i) as *const u8;
            wasm.push(*iter);
            // println!("{}:{:?} : {:?}", i, iter, *iter);
        }
    }
    println!("wasm: {:?}", wasm);

    compile_test(&wasm).unwrap();
    println!("[test]contract compile check pass!");
}

#[allow(dead_code)]
use std::sync::Arc;
use wasmer::wasmparser::Operator;
// use wasmer::Module;

use wasmer::CompilerConfig;
use wasmer::{imports, Instance, Module, Store};
use wasmer_engine_jit::JIT;

use wasmer_compiler_cranelift::Cranelift;
use wasmer_middlewares::Metering;
fn compile_test(code: &[u8]) -> anyhow::Result<()> {
    let gas_limit = 100;
    let cost_function = |operator: &Operator| -> u64 {
        match operator {
            Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
            Operator::I32Add { .. } => 2,
            _ => 0,
        }
    };

    let metering = Arc::new(Metering::new(gas_limit, cost_function));
    let mut compiler_config = Cranelift::default();
    compiler_config.push_middleware(metering);

    let store = Store::new(&JIT::new(compiler_config).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, code)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance
        .exports
        .get_function("add_one")?
        .native::<i32, i32>()?;

    println!("Calling `add_one` function once...");
    assert_eq!(add_one.call(1)?, 2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_x() {
        let code = [
            0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 127, 3, 2, 1, 0, 7, 11, 1, 7, 97,
            100, 100, 95, 111, 110, 101, 0, 0, 10, 9, 1, 7, 0, 32, 0, 65, 1, 106, 11,
        ];
        compile(&code, None).unwrap();
    }

    #[test]
    fn test_y() {
        use wasmer::wat2wasm;
        let code = wat2wasm(
            br#"
(module
  (type $add_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#,
        )
        .unwrap();
        println!("{:?}", code);
        compile(&code, None).unwrap();
    }

    #[test]
    fn test_zz() {
        use wasmer::wat2wasm;
        let code = wat2wasm(
            br#"
(module
  (type $add_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#,
        )
        .unwrap();
        let cost_function = |operator: &Operator| -> u64 {
            match operator {
                Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
                Operator::I32Add { .. } => 2,
                _ => 0,
            }
        };
        let metering = Arc::new(Metering::new(10, cost_function));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);

        let store = Store::new(&JIT::new(compiler_config).engine());

        println!("Compiling module...");
        // Let's compile the Wasm module.
        let module = Module::new(&store, code).unwrap();

        // Create an empty import object.
        let import_object = imports! {};

        println!("Instantiating module...");
        // Let's instantiate the Wasm module.
        let instance = Instance::new(&module, &import_object).unwrap();

        // We now have an instance ready to be used.
        //
        // Our module exports a single `add_one`  function. We want to
        // measure the cost of executing this function.
        let add_one = instance
            .exports
            .get_function("add_one")
            .unwrap()
            .native::<i32, i32>()
            .unwrap();

        println!("Calling `add_one` function once...");
        add_one.call(1).unwrap();
    }
}
