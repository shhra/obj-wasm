use crate::data::*;
use crate::log;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::{FromStr, SplitWhitespace};
use wasm_bindgen::prelude::JsValue;

/// This will read the material properties from the obj file.
pub struct MtlReader<'mat> {
    cur_material: &'mat mut Rc<RefCell<Material>>,
    material_info: &'mat mut HashMap<String, Rc<RefCell<Material>>>,
}

impl<'mat> MtlReader<'mat> {
    pub fn load(
        material: &mut (Rc<RefCell<Material>>, HashMap<String, Rc<RefCell<Material>>>),
        data: &str,
    ) -> Result<(), JsValue> {
        let mut reader = MtlReader {
            cur_material: &mut material.0,
            material_info: &mut material.1,
        };
        reader.parse_mtl(data)?;
        Ok(())
    }

    pub fn parse_mtl(&mut self, data: &str) -> Result<(), JsValue> {
        for line in data.lines() {
            let mut words = line[..].split_whitespace();
            match words.next() {
                Some("#") | None => continue,
                Some("newmtl") => {
                    let name: String = words.map(|x| "".to_owned() + x).collect();
                    self.add_material(name)?;
                },
                Some("Ka") => {
                    let ambient =
                    self.parse_floats(words, &mut self.cur_material.borrow_mut().ambient)?;
                },
                Some("Kd") => {
                    self.parse_floats(words, &mut self.cur_material.borrow_mut().diffuse)?;
                },
                Some("Ks") => {
                    self.parse_floats(words, &mut self.cur_material.borrow_mut().specular)?;
                },
                Some("Tf") => {
                    self.parse_floats(words, &mut self.cur_material.borrow_mut().transmission)?;
                },
                Some("Ni") => {
                    self.parse_single(words, &mut self.cur_material.borrow_mut().ri)?;
                },
                Some("Ns") => {
                    self.parse_single(words, &mut self.cur_material.borrow_mut().shininess)?;
                },
                Some("illum") => {
                    if let Some(p) = words.next() {
                        match FromStr::from_str(p) {
                            Ok(x) => self.cur_material.borrow_mut().illumination = Some(x),
                            Err(_) => return Err(JsValue::from_str("Fetch error")),
                        }
                    } else {
                        return Err(JsValue::from_str("Fetch error"));
                    }
                },
                Some(_) => {}
            }
        }
        Ok(())
    }

    pub fn add_material(&mut self, name: String) -> Result<(), JsValue> {
        let material = match self.material_info.entry(name.clone()) {
            Entry::Occupied(o) => self.material_info.get(&name).unwrap(),
            Entry::Vacant(v) => v.insert(Rc::new(RefCell::new(Material::new()))),
        };
        material.borrow_mut().name = name.clone();
        *self.cur_material =  Rc::clone(material);
        Ok(())
    }

    pub fn parse_floats(&self, words: SplitWhitespace, vals: &mut [f32; 3])
                        -> Result<(), JsValue> {
        let count = words.clone().count();
        if count == 1 {
            for p in words {
                match FromStr::from_str(p) {
                    Ok(x) => vals[0] = x,
                    Err(e) => return Err(JsValue::from_str("Fetch error")),
                }
            }
            vals[1] = 0.0;
            vals[2] = 0.0;
        } else {
            for (i, p) in words.enumerate().take(3) {
                match FromStr::from_str(p) {
                    Ok(x) => vals[i] = x,
                    Err(e) => return Err(JsValue::from_str("Fetch error")),
                }
            }
        }
        Ok(())
    }

    pub fn parse_single(&self, words: SplitWhitespace, vals: &mut f32)
                        -> Result<(), JsValue>
    {
        for p in words {
            match FromStr::from_str(p){
                Ok(x) => *vals = x,
                Err(e) => return Err(JsValue::from_str("Fetch error")),
            }
        }
        Ok(())
    }
}
