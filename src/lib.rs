use std::env::var;
use std::path::PathBuf;

pub mod ac;
pub mod account;
pub mod app;
pub mod constants;
pub mod ctp;
pub mod interface;
pub mod structs;

fn get_interface_path(path: &str) -> PathBuf {
    let px = format!("{}/HFQ/{}", var("HOME").unwrap(), path);
    let path_buffer = PathBuf::from(px);
    if !path_buffer.exists() {
        panic!("please mkdier interface dir fisrt");
    }
    path_buffer.join("bindings.rs")
}
