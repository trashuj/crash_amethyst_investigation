use amethyst::{
    assets::{AssetStorage, Handle, Loader}, //, ProgressCounter},
    core::{
        ecs::{Builder, Entity, WorldExt},
        Transform,
    },
    prelude::*,
    renderer::{
        //palette::{Srgb, Srgba},,
        formats::mesh::ObjFormat,
        mtl::{Material, MaterialDefaults},
        Texture,
        *,
    },
};

use serde_repr::*;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TextureEnum {
    Grass,
    Stone,
    Water,
}

// impl Serialize for TextureEnum {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let index = self as u8;
//         serializer.serialize_u8("ground_texture", index)
//     }
// }

#[derive(Clone)]
struct TextureInfo {
    //progress_counter: ProgressCounter,
    material_handle: Handle<Material>,
}

impl TextureInfo {
    fn new(world: &mut World, texture_file: &str) -> Self {
        let loader = &world.read_resource::<Loader>();
        //let mut progress_counter = ProgressCounter::new();

        let albedo = loader.load(
            format!("texture/{}.png", texture_file),
            ImageFormat::default(),
            //&mut progress_counter,
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );
        // let textures = &world.read_resource();
        // let albedo = loader.load_from_data(
        //     load_from_srgba(Srgba::new(0.3, 0.5, 0.3, 1.0)).into(),
        //     (),
        //     textures,
        // );

        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let materials = &world.read_resource();
        let mat: Handle<Material> = loader.load_from_data(
            Material {
                albedo,
                ..mat_defaults.0.clone()
            },
            (),
            materials,
        );
        Self {
            //progress_counter: progress_counter,
            material_handle: mat,
        }
    }
}

#[derive(Clone)]
pub struct TextureLoader {
    textures: Vec<TextureInfo>,
    mesh_handle: Option<Handle<Mesh>>,
    init: bool,
}

impl TextureLoader {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            mesh_handle: None,
            init: false,
        }
    }

    pub fn init(&mut self, world: &mut World) {
        if !self.init {
            self.init = true;

            self.textures.push(TextureInfo::new(world, "grass"));
            self.textures.push(TextureInfo::new(world, "stone"));
            self.textures.push(TextureInfo::new(world, "water"));

            let loader = &world.read_resource::<Loader>();
            let meshes = &world.read_resource();
            let mesh_handle: Handle<Mesh> =
                loader.load_from("mesh/rectangle.obj", ObjFormat, "", (), &meshes);

            self.mesh_handle = Some(mesh_handle);
        }
    }

    pub fn add_entity(
        &self,
        world: &mut World,
        texture: TextureEnum,
        transform: &Transform,
    ) -> Entity {
        world
            .create_entity()
            .with(self.get_material(texture))
            .with(transform.clone())
            .with(self.mesh_handle.as_ref().unwrap().clone())
            .build()
    }

    pub fn get_material(&self, texture: TextureEnum) -> Handle<Material> {
        self.textures[texture as usize].material_handle.clone()
    }
}
