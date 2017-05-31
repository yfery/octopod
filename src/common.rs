use std::net::TcpListener;
use std::process;
 
// https://rosettacode.org/wiki/Category:Rust
pub fn create_app_lock(port: u16) -> TcpListener {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => {
            socket
        },
        Err(_) => {
            println!("Another instance already running?");
            process::exit(1)
        }
    }
}
 
pub fn remove_app_lock(socket: TcpListener) {
    drop(socket);
}
