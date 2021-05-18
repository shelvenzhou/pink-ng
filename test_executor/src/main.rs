use sc_executor;

use sc_executor::{WasmExecutor, WasmExecutionMethod};
use sp_wasm_interface::{HostFunctions, Function};

pub type SubstrateHostFunctions = (
	sandbox::HostFunctions,
);

fn main() {
    let wasm_method = WasmExecutionMethod::Interpreted;
    let default_heap_pages = Some(17);
    let executor = WasmExecutor::new(
        wasm_method,
        default_heap_pages,
        SubstrateHostFunctions::host_functions(),
        8,
        None,
    );
}
