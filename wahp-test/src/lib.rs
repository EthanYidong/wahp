// cargo build -p wahp-test --target wasm32-unknown-unknown --release

use wahp_core::WahpRequest;

#[link(wasm_import_module = "wahp")]
extern "C" {
    fn request_len() -> u32;
    fn get_request(ptr: *mut u8);
    fn reply(ptr: *const u8, len: u32);
}

#[no_mangle]
fn handle_request() {
    let len = unsafe { request_len() };
    let mut request_bytes = vec![0; len as usize];
    unsafe { get_request(request_bytes.as_mut_ptr()) };

    let request = WahpRequest::from_bytes(&request_bytes);

    let rep = if let Some(name) = request.query.get("me") {
        format!("Hello, {}!", name)
    } else {
        String::from(r#"error: no "me" in query"#)
    };

    unsafe { reply(rep.as_ptr(), rep.len() as u32) };
}
