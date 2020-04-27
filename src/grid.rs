use amethyst::{
    core::{
        ecs::{Builder, Entity, World, WorldExt},
        math::{Point3, Vector3},
        Transform,
    },
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
};
//use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io;
//use std::path::Path;

use crate::texture_loader::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Square {
    #[serde(skip)]
    entity: Option<Entity>,

    ground_texture: Option<TextureEnum>,
}

impl Square {
    pub fn new() -> Self {
        Self {
            entity: None,
            ground_texture: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Grid {
    columns: u16,
    rows: u16,
    squares: Vec<Square>,
}

impl Grid {
    pub fn new(columns: u16, rows: u16, world: &mut World) -> Self {
        let grid = Grid {
            squares: vec![Square::new(); columns as usize * rows as usize],
            columns,
            rows,
        };
        grid.add_debug_display(world);
        grid
    }

    fn add_debug_display(&self, world: &mut World) {
        assert!(self.columns > 0);
        assert!(self.rows > 0);

        // Setup debug lines as a resource
        world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        world.insert(DebugLinesParams { line_width: 1.0 });

        // Setup debug lines as a component and add lines to render axis&grid
        let mut debug_lines_component =
            DebugLinesComponent::with_capacity(self.columns as usize * self.rows as usize);

        let main_color = Srgba::new(0.4, 0.4, 0.4, 1.0);

        // Grid lines in X-axis
        for x in 0..=self.columns {
            let (x, _column, row) = (x as f32, self.columns as f32, self.rows as f32);

            let position = Point3::new(x - 0.5, 0.0, -0.5);
            let direction = Vector3::new(0.0, 0.0, row);

            debug_lines_component.add_direction(position, direction, main_color);
        }

        // Grid lines in Z-axis
        for z in 0..=self.rows {
            let (z, column, _rows) = (z as f32, self.columns as f32, self.rows as f32);

            let position = Point3::new(-0.5, 0.0, z - 0.5);
            let direction = Vector3::new(column, 0.0, 0.0);

            debug_lines_component.add_direction(position, direction, main_color);
        }

        // // add x, y z axis
        // let origin = Point3::new(0.0, 0.0, 0.0);
        // debug_lines_component.add_direction(
        //     origin,
        //     Vector3::new(1.0, 0.0, 0.0),
        //     Srgba::new(1.0, 0.0, 0.0, 1.0),
        // );
        // debug_lines_component.add_direction(
        //     origin,
        //     Vector3::new(0.0, 1.0, 0.0),
        //     Srgba::new(0.0, 1.0, 0.0, 1.0),
        // );
        // debug_lines_component.add_direction(
        //     origin,
        //     Vector3::new(0.0, 0.0, 1.0),
        //     Srgba::new(0.0, 0.0, 1.0, 1.0),
        // );

        world.register::<DebugLinesComponent>();
        world.create_entity().with(debug_lines_component).build();
    }

    pub fn load(
        &mut self,
        world: &mut World,
        texture_loader: &TextureLoader,
        file: &str,
    ) -> Result<(), io::Error> {
        println!("load {}", file);

        let file = File::open(file)?;
        let grid: Grid = serde_json::from_reader(&file).unwrap();

        for square in &mut self.squares {
            assert!(square.ground_texture.is_none());
            assert!(square.entity.is_none());
        }
        // TODO: works only for same # of col/rows
        self.columns = grid.columns;
        self.rows = grid.rows;

        let size = grid.columns * grid.rows;
        for i in 0..size {
            let square = &grid.squares[i as usize];
            if let Some(ground_texture) = &square.ground_texture {
                let square = &mut self.squares[i as usize];
                square.ground_texture = Some(*ground_texture);

                let x = (i % grid.columns) as f32;
                let y = (i / grid.columns) as f32;

                // println!(
                //     "adding ground for square {}, pos {}, {}, texture = {}",
                //     i, x, y, *ground_texture as u8
                // );

                let mut transform = Transform::default();
                transform.set_scale(Vector3::new(0.5, 1.0, 0.5));
                transform.append_translation_xyz(x, 0.0, y);
                square.entity = Some(texture_loader.add_entity(world, *ground_texture, &transform));
            }
        }

        Ok(())
    }
}
