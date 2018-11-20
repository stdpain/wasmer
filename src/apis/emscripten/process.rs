use libc::{
    // c_int,
    // c_void,
    c_char,
    // size_t,
    // ssize_t,
    abort,
};

use std::ffi::CStr;
use crate::webassembly::{Instance};

extern "C" fn abort_with_message(message: &str) {
    println!("{}", message);
    _abort();
}

/// emscripten: _abort
pub extern "C" fn _abort() {
    unsafe { abort(); }
}

/// emscripten: abort
pub extern "C" fn em_abort(message: u32, instance: &mut Instance) {
    let message_addr = instance.memory_offset_addr(0, message as usize) as *mut c_char;
    unsafe {
        let message = CStr::from_ptr(message_addr)
            .to_str()
            .unwrap_or("Unexpected abort");

        abort_with_message(message);
    }
}

/// emscripten: abortOnCannotGrowMemory
pub extern "C" fn abort_on_cannot_grow_memory() {
    abort_with_message("Cannot enlarge memory arrays!");
}

