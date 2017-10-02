extern crate bindgen;
extern crate which;

use std::env;
use std::path::PathBuf;

use which::which;

fn find_node_api_header() -> Result<String, &'static str> {
    Ok(
        which("node")?
            .as_path()
            .parent()
            .ok_or("Could not go up from Node.js executable location")?
            .parent()
            .ok_or("Could not go up from Node.js bin directory")?
            .join("include")
            .join("node")
            .join("node_api.h")
            .to_str()
            .ok_or("Could not convert path to string")?
            .to_owned(),
    )
}

fn main() {
    let header_path = find_node_api_header().unwrap();

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
