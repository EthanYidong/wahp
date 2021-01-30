use wasmer::*;

pub enum CopyType<'a> {
    FromWasmMemory(&'a mut [u8]),
    ToWasmMemory(&'a [u8]),
}

impl CopyType<'_> {
    pub fn len(&self) -> usize {
        match self {
            CopyType::FromWasmMemory(v) => v.len(),
            CopyType::ToWasmMemory(v) => v.len(),
        }
    }
}

pub fn checked_copy(memory: &Memory, wasm_ptr: i32, copy: CopyType) {
    let copy_len = copy.len();
    let copy_end = wasm_ptr + copy_len as i32;

    if wasm_ptr < 0 || copy_end > memory.data_size() as i32  {
        panic!("Out of bounds: range from {} to {}, max {}", wasm_ptr, copy_end, memory.data_size());
    }

    match copy {
        CopyType::FromWasmMemory(dest) => {
            unsafe { std::ptr::copy_nonoverlapping(memory.data_ptr().add(wasm_ptr as usize), dest.as_mut_ptr(), copy_len) }
        },
        CopyType::ToWasmMemory(src) => {
            unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), memory.data_ptr().add(wasm_ptr as usize), copy_len) }
        },
    }
}
