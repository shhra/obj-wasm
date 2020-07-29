# Obj-Wasm

This is a tiny library that loads obj files in the browser. The objective of this libary is to assists 
loading of "obj" and "mtl" files through wasm. Since wasm doesn't support file operations, the libary
relies on getting the data from javascript. Therefore it relies on the fetch operation provided by the 
browser. Current features include:

1. Parsing the "obj" and "mtl" files.
2. Creating a basic tree from the ".obj" file.

The idea in this libary is dead simple and further features are on the hold. 

  * **TODO**
  
    [ ] Load textures materials.

    [ ] Support indexing to load the vertices.

    [ ] Better support for error handling.

This library is heavily inpsired by [Assimp](https://github.com/assimp/assimp) and [tobj]( https://github.com/Twinklebear/tobj ) 



## How to use

```rust

use obj::reader::*;

let mut reader = ObjReader::new()?;
reader.read(&self.scene_data, &self.mat_data);

```

Reader will now have the a your scene graph. You can access different data from the scene graph.
