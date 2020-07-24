#![allow(unused)]
pub mod utils;
pub mod reader;
pub mod data;
pub mod parser;
pub mod scene;
pub mod mtlreader;

use wasm_bindgen::prelude::*;
use reader::ObjReader;
use web_sys::*;
use data::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Log for easier debugging and displaying in web-console.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
pub fn load_scene(data: &str) {
    utils::set_panic_hook();
    let reader = ObjReader::new().unwrap();
    let face = Face::new();
    let material = Material::new();
    let mesh = Mesh::new();
    let model = Model::new();
    let object = Object::new();

    log!("{:?}", reader.scene);
    log!("This should load the default object file");
}

#[wasm_bindgen]
pub fn load_material(data: &str) {
    log!("Loading the material data below:\n {}", data)
}

#[wasm_bindgen]
pub fn greet() {
    utils::set_panic_hook();
    log!("Hello");
}
