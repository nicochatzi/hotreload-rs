#[no_mangle]
pub fn print() {
    println!("ya");
}

#[no_mangle]
pub fn wait() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}
