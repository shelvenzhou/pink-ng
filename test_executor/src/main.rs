use sc_executor::{self, CallInWasm};

use sc_executor::{WasmExecutionMethod, WasmExecutor};
use sp_runtime_interface::runtime_interface;
use sp_wasm_interface::{Function, HostFunctions};

#[runtime_interface(no_tracing)]
trait MyInterface {
    fn say_hello_world(data: &str) {
        println!("Hello world from: {}", data);
    }
}

fn main() {
    let executor = WasmExecutor::new(
        WasmExecutionMethod::Interpreted,
        Some(17),
        my_interface::HostFunctions::host_functions(),
        8,
        None,
    );


    let wasm_binary: [u8; 10] = [0; 10];
    let call_data: [u8; 1] = [0];

    executor.call_in_wasm(
        &wasm_binary,
        None,
        "my_function",
        &call_data,
        ,
        sp_core::traits::MissingHostFunctions::Allow,
    );
}
