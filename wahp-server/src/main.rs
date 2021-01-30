mod wasm_memory;
mod wahp_env;
mod wasm_runner;

use wasmer::*;

use warp::Filter;

use std::collections::HashMap;

use wahp_env::*;
use wasm_runner::WasmRunnerHandle;

async fn handle(query: HashMap<String, String>, runner: WasmRunnerHandle) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(String::from_utf8(runner.send_request(wahp_core::WahpRequest {
        query,
    }.to_bytes()).await).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = WasmRunnerHandle::start_runner();
    let filter = warp::query::<HashMap<String, String>>()
        .and(warp::any().map(move || runner.clone()))
        .and_then(handle);

    warp::serve(filter)
        .bind(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
