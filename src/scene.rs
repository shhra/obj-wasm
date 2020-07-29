use crate::data::*;
use std::rc::Rc;
use cgmath::{Matrix4, Vector3};

/// Hold index.
#[derive(Debug, Clone)]
pub struct Index {
    pub index: usize,
}

/// Defines the face and becomes an interface to access the face information.
/// It holds the indices to access the faces present within the mesh.
#[derive(Debug, Clone)]
pub struct SceneFace { // Is questionable? Do I need it?
    /// List of indices pointing to the vertices, normals and textures.
    pub indices: Vec<usize>,
}

impl SceneFace {
    pub fn new() -> Self {
        SceneFace {
            indices: Vec::new(),
        }
    }

}

/// Store vertices, normals, textures, colors, and list of faces.
#[derive(Debug, Clone)]
pub struct SceneMesh {
    /// Name of the
    pub name: String,
    /// List of vertices
    pub gv: Vec<f32>,
    /// List of normals
    pub vn: Vec<f32>,
    /// List of textures
    pub vt: Vec<f32>,
    /// List of colors
    pub colors: Vec<f32>,
    /// Material assigned to this mesh.
    pub material: Option<Index>,
    /// list of faces.
    pub faces: Vec<SceneFace>,
    /// All face indices.
    pub face_indices: Vec<usize>,
}

impl SceneMesh {
    pub fn new() -> Self {
        SceneMesh {
            name: String::new(),
            gv: Vec::new(),
            vn: Vec::new(),
            vt: Vec::new(),
            colors: Vec::new(),
            material: None,
            faces: Vec::new(),
            face_indices: Vec::new(),
        }
    }
}


/// Stores the information about the node.
#[derive(Debug, Clone)]
pub struct SceneNode {
    /// A reference to the parent node.
    pub parent: Option<Index>,
    /// A list containing child nodes.
    pub children: Vec<usize>,
    /// Name to define the give node.
    pub name: String,
    /// Stores transformation of the given node.
    pub transformation: Matrix4<f32>,
    /// List of indices to the SceneMesh stored in the SceneGraph
    pub meshes: Vec<usize>,
}

impl SceneNode {
    pub fn new() -> Self {
        SceneNode {
            name: String::new(),
            transformation: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            parent: None,
            children: Vec::new(),
            meshes: Vec::new(),
        }
    }
}

/// The entire graph of the scene along with some data for other points to access.
#[derive(Debug)]
pub struct SceneGraph {
    /// A root node.
    pub nodes: Vec<SceneNode>,
    /// List of meshes.
    pub meshes: Vec<SceneMesh>,
    /// List of material
    pub materials: Vec<Material>,
}

impl SceneGraph {
    pub fn new() -> Self {
        SceneGraph {
            nodes: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn owned_nodes(&self) -> Vec<SceneNode> {
        self.nodes.to_owned()
    }

    pub fn owned_meshes(&self) -> Vec<SceneMesh> {
        self.meshes.to_owned()
    }

    pub fn owned_materials(&self) -> Vec<Material> {
        self.materials.to_owned()
    }
}
