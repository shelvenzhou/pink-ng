use sc_executor;

use sc_executor::{WasmExecutor, WasmExecutionMethod};
use sp_wasm_interface::{HostFunctions, Function};
use sp_runtime_interface::runtime_interface;


#[runtime_interface(no_tracing)]
trait MyInterface {
    fn say_hello_world(data: &str) {
        println!("Hello world from: {}", data);
    }
}

fn main() {

    let wasm_method = WasmExecutionMethod::Interpreted;
    let default_heap_pages = Some(17);
    let executor = WasmExecutor::new(
        wasm_method,
        default_heap_pages,
        my_interface::HostFunctions::host_functions(),
        8,
        None,
    );
}
