use crate::GLOBAL;
use std::sync::MutexGuard;

pub unsafe fn store() {
    println!("GLOBAL: {}", GLOBAL.lock().unwrap());
}

pub fn get_global() -> MutexGuard<'static, i128> {
    GLOBAL.lock().unwrap()
}

pub fn set_global(new_global: i128) {
    *GLOBAL.lock().unwrap() = new_global;
}
