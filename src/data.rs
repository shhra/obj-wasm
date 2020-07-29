/// Contains data structures that holds the parsed information from the file.
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use cgmath::{Matrix4, Vector3};

#[derive(Clone, Debug)]
pub enum FaceType {
    Point,
    Line,
    Triangle,
    Polygon,
}

type IndexArray = Vec<usize>;

/// Holds different information about the materials present within the scene.
#[derive(Clone, Debug)]
pub struct Material {
    /// material name
    pub name: String,
    /// name of the texture
    pub texture: String,
    /// name of the ambient texture
    pub texture_ambient: String,
    /// name of the diffuse texture
    pub texture_diffuse: String,
    /// name of the emissive texture
    pub texture_emissive: String,
    /// name of the normal texture
    pub texture_normal: String,
    /// name of the specular texture
    pub texture_specular: String,
    /// ambient color
    pub ambient: [f32; 3],
    /// diffuse color
    pub diffuse: [f32; 3],
    /// emissive color
    pub transmission: [f32; 3],
    /// specular color
    pub specular: [f32; 3],
    /// shininess factor:
    pub shininess: f32,
    /// illumination model
    pub illumination: Option<u8>,
    /// refractive index
    pub ri: f32,

}

/// Holds the information about the mesh.
#[derive(Clone, Debug)]
pub struct Mesh {
    /// The name for the mesh
    pub name: String,
    /// Array with all stored faces
    pub faces: Vec<Face>,
    /// Assigned material
    pub material: Rc<RefCell<Material>>,
    /// Number of stored indices.
    pub num_indices: u32,
    /// True, if normals are stored.
    has_normals: bool,
    /// True, if vertex colors are stored.
    has_vertex_colors: bool,
}

/// Holds the structure for face
#[derive(Clone, Debug)]
pub struct Face {
    /// FaceType
    pub face_type: FaceType,
    /// Vertex indices
    pub vertices: IndexArray,
    /// Normal indices
    pub normals: IndexArray,
    /// Texture coordinates indices
    pub textures: IndexArray,
    /// Pointer to assigned material
    pub material: Rc<RefCell<Material>>,
}

/// Data structure to hold the objects present withing the mesh file.
#[derive(Clone, Debug)]
pub struct Object {
    /// Name of the object
    pub name: String,
    /// Transformations
    pub transform: Matrix4<f32>,
    /// Points to sub objects within this object.
    pub sub_objects: Vec<usize>,
    /// Assigned meshes
    pub meshes: Vec<usize>,
}

/// Data Structure to hold model
#[derive(Clone, Debug)]
pub struct Model {
    // TODO: Implement group later.
    /// Model name
    pub model_name: String,
    /// List ob assigned objects
    pub objects: Vec<Object>,
    /// Pointer to current object
    pub cur_obj: usize,
    /// Pointer to current material
    pub matlib: (Rc<RefCell<Material>>, HashMap<String, Rc<RefCell<Material>>>),
    /// Pointer to default material
    pub default_material: Option<Material>,
    /// map of group library (active group name, a dictionary of list of faces.)
    pub grplib: (String, HashMap<String, Vec<usize>>),
    /// Vector with all generated vertices
    pub gv: Vec<f32>,
    /// vector with all generated normals
    pub vn: Vec<f32>,
    /// vector with all textures
    pub vt: Vec<f32>,
    /// vector with all vertex colors
    pub colors: Vec<f32>,
    /// Current mesh instance
    pub cur_mesh: usize,
    /// Vector with stored meshes
    pub meshes: Vec<Mesh>,
}

impl Material {
    /// Create a new material for parsing
    pub fn new() -> Self {
        Material {
            name: String::new(),
            texture: String::new(),
            texture_ambient: String::new(),
            texture_diffuse: String::new(),
            texture_emissive: String::new(),
            texture_normal: String::new(),
            texture_specular: String::new(),
            ambient: [0.0; 3],
            diffuse: [0.0; 3],
            transmission: [0.0; 3],
            specular: [0.0; 3],
            shininess: 0.0,
            illumination: None,
            ri: 1.0,
        }
    }
}

impl Mesh {
    /// Create a new mesh for parsing.
    pub fn new() -> Self {
        Mesh {
            name: String::new(),
            faces: Vec::new(),
            material: Rc::new(RefCell::new(Material::new())),
            num_indices: 0,
            has_normals: false,
            has_vertex_colors: false,
        }
    }
}

impl Face {
    /// Create a new face for parsing.
    pub fn new() -> Self
    {
        Face {
            face_type: FaceType::Triangle,
            vertices: IndexArray::new(),
            normals: IndexArray::new(),
            textures: IndexArray::new(),
            material: Rc::new(RefCell::new(Material::new())),
        }
    }
}

impl Object {
    /// Create a object for parsing.
    pub fn new() -> Self {
        Object {
            name: String::new(),
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            sub_objects: Vec::new(),
            meshes: Vec::new(),
        }
    }
}

impl Model {
    /// Create a model for parsing.
    pub fn new() -> Self {
        Model {
            model_name: String::new(),
            objects: Vec::new(),
            cur_obj: 0,
            matlib: (Rc::new(RefCell::new(Material::new())), HashMap::new()),
            default_material: None,
            grplib: (String::new(), HashMap::new()),
            gv: Vec::new(),
            vn: Vec::new(),
            vt: Vec::new(),
            colors: Vec::new(),
            cur_mesh: 0,
            meshes: Vec::new(),
        }
    }
}
