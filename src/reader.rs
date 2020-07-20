/// This class will read the file and creates a scene.
///
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, Response};

use std::rc::Rc;
use crate::log;
use crate::parser::*;


#[wasm_bindgen]
pub struct DataLoader {
    reader: ObjReader,
    mat_data: String,
    scene_data: String,
}

#[wasm_bindgen]
impl DataLoader {
    pub fn new() -> Result<DataLoader, JsValue> {
        let loader =  DataLoader {
            reader: ObjReader::new()?,
            mat_data: String::new(),
            scene_data: String::new(),
        };
        Ok(loader)
    }
    pub fn load_scene(&mut self, data: String) {
        self.scene_data = data;
    }

    pub fn load_material(&mut self, data: String) {
        self.mat_data = data;
    }

    pub fn load(&mut self) -> Result<(), JsValue> {
        log!("loading scene");
        self.reader.read(&self.scene_data, &self.mat_data)?;
        Ok(())
    }
}


pub struct ObjReader {
    pub scene: String
}


impl ObjReader{

    pub fn new () -> Result<ObjReader, JsValue> {
        let mut result = ObjReader {
            scene: String::from("Good work")
        };
        Ok(result)
    }

    pub fn read(&mut self, scene_data: &str, mat_data: &str) -> Result<(), JsValue> {
        log!("Populating Scene");
        let parser = Parser::parse(scene_data, mat_data)?;
        let model = &parser.model.borrow();
        log!("Model name {:#?}", model.model_name);
        log!("Total objects: {:#?}", model.objects.len());
        log!("Current Object: {:#?}", model.objects[model.cur_obj]);
        log!("Current Material: {:#?}", model.matlib.0.borrow().name);
        log!("Default Material: {:#?}", model.default_material);
        log!("Active group: {:#?}", model.grplib.0);
        log!("Vertex length: {:#?}", model.gv.len());
        log!("Texture length: {:#?}", model.vt.len());
        log!("Normal length: {:#?}", model.vn.len());
        log!("Colors length: {:#?}", model.colors.len());
        log!("Current Mesh: {:#?}", model.meshes[model.cur_mesh]);
        log!("Total Meshes: {:#?}", model.meshes.len());
        log!("Material libray: {:#?}", model.matlib.1.len());
        Ok(())
    }
}
