use wasmer::*;

use tokio::sync::mpsc::UnboundedSender;

use std::sync::Arc;

use crate::wasm_memory::*;

#[derive(WasmerEnv, Debug, Clone)]
pub struct WahpEnv {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
    request_bytes: Arc<Vec<u8>>,
    rx: UnboundedSender<Vec<u8>>,
}

impl WahpEnv {
    pub fn new(request_bytes: Vec<u8>, rx: UnboundedSender<Vec<u8>>) -> WahpEnv {
        WahpEnv {
            memory: LazyInit::new(),
            request_bytes: Arc::new(request_bytes),
            rx,
        }
    }

    pub fn get_imports(self, store: &Store) -> ImportObject {
        imports!{
            "wahp" => {
                "request_len" => Function::new_native_with_env(&store, self.clone(), request_len),
                "get_request" => Function::new_native_with_env(&store, self.clone(), get_request),
                "reply" => Function::new_native_with_env(&store, self.clone(), reply),
            }
        }
    }
}

pub fn request_len(env: &WahpEnv) -> u32 {
    env.request_bytes.len() as u32
}

pub fn get_request(env: &WahpEnv, ptr: i32) {
    let memory = env.memory.get_ref().unwrap();

    checked_copy(memory, ptr, CopyType::ToWasmMemory(&env.request_bytes));
}

pub fn reply(env: &WahpEnv, ptr: i32, len: u32) {
    let memory = env.memory.get_ref().unwrap();

    let mut reply = vec![0; len as usize];
    checked_copy(memory, ptr, CopyType::FromWasmMemory(&mut reply));

    env.rx.send(reply);
}
