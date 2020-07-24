/// This class will read the file and creates a scene.
///
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, Response};

use std::rc::Rc;
use std::cell::{RefCell, Ref};
use crate::log;
use crate::data::*;
pub use crate::parser::*;
pub use crate::scene::*;


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
    pub scene: SceneGraph,
}


impl ObjReader{

    pub fn new () -> Result<ObjReader, JsValue> {
        let mut result = ObjReader {
            scene: SceneGraph::new()
        };
        Ok(result)
    }

    pub fn read(&mut self, scene_data: &str, mat_data: &str) -> Result<(), JsValue> {
        log!("Populating Scene");
        let parser = Parser::parse(scene_data, mat_data)?;
        let model = parser.model.borrow();
        self.fill_nodes(&model)?;
        Ok(())
    }

    pub fn fill_nodes(&mut self, model: &Ref<Model>) -> Result<(), JsValue>
    {
        let mut root = SceneNode::new();
        self.scene.nodes.push(root);
        let root_index = self.scene.nodes.len() - 1;
        let mut children: Vec<usize> = Vec::new();
        let mut index: usize = 0;
        self.load_materials(&model);
        for obj in model.objects.iter() {
            let mut node = SceneNode::new();
            node.name = obj.name.clone();
            node.parent = Some(Index { index: root_index});
            self.load_meshes(&model, &obj, &mut node, &root_index);
            self.scene.nodes.push(node);
            if obj.sub_objects.len() > 0 {
                index += 1 as usize;
                children.push(index.clone())
            }
        }
        if children.len() > 0 {
            for i in children.iter() {
                let obj = &model.objects[i.clone()];
                for child in obj.sub_objects.iter() {
                    self.scene.nodes[child.clone()].parent = Some(Index {index: (i + 1 as usize)})
                }
            }
        }
        // TODO: Load materials
        Ok(())
    }

    pub fn load_materials(&mut self, model:&Ref<Model>) {
        for (material_name, material) in &model.matlib.1 {
            self.scene.material.push(material.borrow().clone());
        }

    }

    pub fn load_meshes(&mut self, model: &Ref<Model>, obj:&Object, node: &mut SceneNode, root:&usize) {
        for mesh in obj.meshes.iter() {
            let model_mesh =  &model.meshes[mesh.clone()];
            let filled_mesh = self.filled_mesh(&model, &model_mesh);
            let mesh_index = self.scene.meshes.len();
            node.meshes.push(mesh_index.clone());
            self.scene.meshes.push(filled_mesh);
        }

    }

    pub fn filled_mesh(&self, model: &Ref<Model>, mesh:&Mesh) -> SceneMesh {
        let mut scene_mesh = SceneMesh::new();
        scene_mesh.name = mesh.name.clone();
        let mut new_index:usize = 0;
        for face in mesh.faces.iter() {
            // Use the face to fetch vertices.
            let mut new_face = SceneFace::new();
            let b: u32 = face.vertices.len() as u32;
            for idx in 0..b {
                let index = idx as usize;
                scene_mesh.gv.push(model.gv[face.vertices[index] * 3]);
                scene_mesh.gv.push(model.gv[face.vertices[index] * 3 + 1 as usize]);
                scene_mesh.gv.push(model.gv[face.vertices[index] * 3 + 2 as usize]);
                // TODO: If vt empty
                scene_mesh.vt.push(model.vt[face.textures[index] * 2]);
                scene_mesh.vt.push(model.vt[face.textures[index] * 2+ 1 as usize]);
                // TODO: If vn empty
                scene_mesh.vn.push(model.vn[face.normals[index] * 3 ]);
                scene_mesh.vn.push(model.vn[face.normals[index] * 3 + 1 as usize]);
                scene_mesh.vn.push(model.vn[face.normals[index] * 3 + 2 as usize]);
                new_face.indices.push(new_index);
                new_index += 1 as usize;
            }
            scene_mesh.faces.push(new_face);
        }

        let material = &mesh.material.borrow();
        let mut mat_iter = self.scene.material.iter();
        let index = mat_iter.position(|x| x.name == material.name);
        match index {
            Some(x) => scene_mesh.material = Some(Index {index: x}),
            None => ()
        }
        scene_mesh

    }
}
