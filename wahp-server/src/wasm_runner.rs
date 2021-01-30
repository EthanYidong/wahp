use tokio::sync::*;

use wasmer::*;

use crate::wahp_env::WahpEnv;

#[derive(Clone)]
pub struct WasmRunnerHandle {
    sender: mpsc::Sender<(Vec<u8>, oneshot::Sender<Vec<u8>>)>
}

impl WasmRunnerHandle {
    pub fn start_runner() -> WasmRunnerHandle {
        let (tx, mut rx) = mpsc::channel::<(Vec<u8>, oneshot::Sender<Vec<u8>>)>(32);
    
        tokio::spawn(async move {
            let store = Store::default();
            let module = Module::from_file(&store, &"target/wasm32-unknown-unknown/release/wahp_test.wasm").unwrap();
            
            while let Some((data, tx_reply)) = rx.recv().await {
                let (tx_data, mut rx_data) = mpsc::unbounded_channel();
                {
                    let env = WahpEnv::new(data, tx_data);
                    let imports = env.get_imports(&store);
                    let instance = Instance::new(&module, &imports).unwrap();
                    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("handle_request").unwrap();
                    run_func.call().unwrap();
                }

                tx_reply.send(rx_data.recv().await.unwrap());
            }
        });
    
        WasmRunnerHandle {
            sender: tx,
        }
    }

    pub async fn send_request(&self, data: Vec<u8>) -> Vec<u8> {
        let (tx, rx) = oneshot::channel();

        self.sender.send((data, tx)).await.unwrap();

        rx.await.unwrap()
    }
}
