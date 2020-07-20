use std::cell::RefCell;
use std::fmt::Error;
/// This class will handle the parsing of scene.
/// from a string
use std::rc::Rc;
use std::str::{FromStr, SplitWhitespace};
use std::collections::HashMap;
use wasm_bindgen::prelude::JsValue;
use std::collections::hash_map::Entry;

use crate::data::*;
use crate::log;
use crate::mtlreader::MtlReader;

pub struct Parser {
    pub model: Rc<RefCell<Model>>,
}

impl Parser {
    pub fn parse(scene_data: &str, mat_data:&str) -> Result<Parser, JsValue> {
        let mut parser = Parser {
            model: Rc::new(RefCell::new(Model::new())),
        };
        parser.read_data(scene_data, mat_data)?;
        Ok(parser)
    }

    fn read_data(&mut self, scene_data: &str, mat_data:&str) -> Result<(), JsValue> {
        let mut model = self.model.borrow_mut();
        for line in scene_data.lines() {
            let mut words = line[..].split_whitespace();
            match words.next() {
                Some("#") | None => continue,
                Some("mtllib") => {
                    let name: String = words.map(|x| " ".to_owned() + x).collect();
                    log!("Material Name: {}", &name);
                    MtlReader::load(&mut model.matlib, mat_data);
                },
                Some("v") => {
                    if words.clone().count() <= 4 {
                        self.parse_floats(words, &mut model.gv)?;
                    } else if words.clone().count() == 6 {
                        log!("This is not implemented")
                        // self.parse_with_colors(words, &mut model.gv, &mut model.colors)?;
                    }
                },
                Some("vt") => {
                    self.parse_floats(words, &mut model.vt)?;
                },
                Some("vn") => {
                    self.parse_floats(words, &mut model.vn)?;
                },
                Some("f") => {
                    self.parse_face(words, &mut model);
                },
                Some("g") => {
                    self.parse_group(words, &mut model)?;
                },
                Some("o") => {
                    self.parse_obj(words, &mut model)?;
                },
                Some("usemtl") => {
                    self.use_material(words, &mut model)?;
                }

                Some(_) => {}
            }
        }
        Ok(())
    }

    fn parse_floats(&self, words: SplitWhitespace, vals: &mut Vec<f32>)
                        -> Result<(), JsValue> {
        let count = words.clone().count();
        if count == 3 {
            for p in words {
                match FromStr::from_str(p) {
                    Ok(x) => vals.push(x),
                    Err(e) => return Err(JsValue::from_str("Fetch error")),
                }
            }
        } else if count == 4 {
            let mut temp: Vec<f32> = Vec::new();
            for p in words {
                match FromStr::from_str(p) {
                    Ok(x) => temp.push(x),
                    Err(e) => return Err(JsValue::from_str("Fetch error")),
                }
            }
            let w = temp[3];
            vals.push(temp[0] / &w);
            vals.push(temp[1] / &w);
            vals.push(temp[2] / &w);
        } else {
            for p in words {
                match FromStr::from_str(p) {
                    Ok(x) => vals.push(x),
                    Err(e) => return Err(JsValue::from_str("Fetch error")),
                }
            }
        }
        Ok(())
    }

    fn parse_with_colors(&self,
                             words: SplitWhitespace,
                             data: &mut Vec<f32>,
                             color: &mut Vec<f32>,
    ) -> Result<(), JsValue> {
        // Not implemented yet.
        Ok(())
    }

    fn parse_face(&self, words: SplitWhitespace, model: &mut Model) -> Result<(), JsValue> {
        // Make it work for points and lines too!
        let mut face: Face = Face::new();
        face.face_type = FaceType::Triangle;
        for each in words {
            for (idx, data) in each.split("/").enumerate() {
                if !data.is_empty() {
                    match usize::from_str(data) {
                        Ok(x) => if x < 0 {
                            match idx {
                                0 => face.vertices.push(x + model.gv.len() / 3),
                                1 => face.textures.push(x + model.gv.len() / 2),
                                2 => face.normals.push(x + model.gv.len() / 3),
                                _ => panic!("Error while parsing face")
                            }
                        } else {
                            match idx {
                                0 => face.vertices.push(x - 1),
                                1 => face.textures.push(x - 1),
                                2 => face.normals.push(x - 1),
                                _ => panic!("Error while parsing face")
                            }
                        },
                        Err(_) => return Err(JsValue::from_str("No face for you!"))
                    }
                }
           }
        }

        if face.vertices.len() > 3 {
            let mut temp = Vec::new();
            let a = face.vertices[0];
            let mut b = face.vertices[1];
            for c in face.vertices.iter().skip(2) {
                temp.push(a);
                temp.push(b);
                temp.push(*c);
                b = *c;
            }
            face.vertices = temp;
        }
        if face.normals.len() > 3 {
            let mut temp = Vec::new();
            let a = face.normals[0];
            let mut b = face.normals[1];
            for c in face.normals.iter().skip(2) {
                temp.push(a);
                temp.push(b);
                temp.push(*c);
                b = *c;
            }
            face.normals = temp;
        }
        if face.textures.len() > 3 {
            let mut temp = Vec::new();
            let a = face.textures[0];
            let mut b = face.textures[1];
            for c in face.textures.iter().skip(2) {
                temp.push(a);
                temp.push(b);
                temp.push(*c);
                b = *c;
            }
            face.textures = temp;
        }

        face.material = Rc::clone(&model.matlib.0);
        model.meshes[model.cur_mesh].faces.push(face);
        Ok(())
    }

    fn parse_group(&self, words: SplitWhitespace, model: &mut Model) -> Result<(), JsValue>
    {
        for x in words {
            if &model.grplib.0 == x {
                return Ok(())
            }
            let group_face = match model.grplib.1.entry(x.to_string()) {
                Entry::Occupied(o) => model.grplib.1.get(&x.to_string()).unwrap(),
                Entry::Vacant(v) => v.insert(Vec::new()),
            };
            self.create_object(x, model);
            model.grplib.0 = x.to_string();
        }
        Ok(())
    }

    fn parse_obj(&self, words: SplitWhitespace, model: &mut Model) -> Result<(), JsValue>
    {

        let mut iter = model.objects.iter();
        let name: String = words.collect();
        let obj_name = iter.position(|x| x.name == name );
        match obj_name {
            Some(o) => model.cur_obj = o,
            None => self.create_object(&name, model)?,
        }
        Ok(())

    }

    fn create_object(&self, name: &str, model: &mut Model) -> Result<(), JsValue> {
        let mut iter = model.objects.iter();
        let obj_name = iter.position(|x| &x.name == name );
        match obj_name {
            Some(x) => model.cur_obj = x,
            None => {
                let mut obj = Object::new();
                obj.name = name.to_string();
                model.objects.push(obj);
                self.create_mesh(name, model);
            }
        }
        model.meshes[model.cur_mesh].material = Rc::clone(&model.matlib.0);

        Ok(())
    }

    fn create_mesh(&self, name: &str, model: &mut Model) -> Result<(), JsValue> {
        let mut mesh = Mesh::new();
        mesh.name = name.to_string();
        model.meshes.push(mesh);
        let index = model.meshes.len() - 1 as usize;
        model.objects[model.cur_obj].meshes.push(index);
        Ok(())
    }

    fn use_material(&self, word: SplitWhitespace, model: &mut Model) -> Result<(), JsValue> {
        let name:String = word.collect();
        log!("Loading material: {}", name);
        let material = match model.matlib.1.entry(name.clone()) {
            Entry::Occupied(o) => model.matlib.1.get(&name).unwrap(),
            Entry::Vacant(v) => {
                model.matlib.1.get("initialShadingGroup").unwrap()
                // let message = String::from("The material doesn't exists ") + &name;
                // return Err(JsValue::from_str(&message));
            }
        };
        model.matlib.0 = material.clone();
        log!("Loaded {:?}", model.matlib.0.borrow().name);
        Ok(())

    }


}