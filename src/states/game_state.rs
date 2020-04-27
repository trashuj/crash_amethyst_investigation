use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::{
        ecs::{prelude::Entity, Builder, WorldExt},
        Transform,
    },
    input::is_close_requested,
    prelude::*,
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb,
        *,
    },
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::grid::*;
use crate::states::{MainMenuState, *};
use crate::texture_loader::*;

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform
        .set_translation_xyz(5.0, 5.0, 30.0)
        .set_rotation_x_axis(5.0);
    transform.move_up(25.0);
    world
        .create_entity()
        .with(Camera::standard_3d(1440.0, 900.0))
        .with(transform)
        .build();
}

fn _initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world.create_entity().with(light).with(transform).build();
}

pub struct GameState {
    grid: Option<Grid>,
    texture_loader: TextureLoader,

    prefab_progress_counter: ProgressCounter,
    prefab: Option<Handle<Prefab<MyPrefabData>>>,
}

impl GameState {
    pub fn new(texture_loader: TextureLoader) -> Self {
        Self {
            grid: None,
            texture_loader: TextureLoader::new(),
            prefab_progress_counter: ProgressCounter::new(),
            prefab: None,
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        //let fetched = world.try_fetch::<TextureLoader>();

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("prefab/game_ui.ron", ());
        });

        initialize_camera(world);

        self.grid = Some(Grid::new(10, 8, world));
        self.texture_loader.init(world);

        self.prefab = Some(world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load(
                "prefab/scene.ron",
                RonFormat,
                &mut self.prefab_progress_counter,
            )
        }));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.prefab_progress_counter.is_complete() {
            if let Some(prefab) = &self.prefab {
                println!("loading scene");
                world.create_entity().with(prefab.clone()).build();

                self.prefab = None;

                if let Some(grid) = &mut self.grid {
                    let file = "assets/map/bridge.json";
                    if !grid.load(world, &self.texture_loader, file).is_ok() {
                        println!("failed to load file {}", file);
                    }
                }
            }
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        state_data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    println!("Bye bye!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                let mut exit_button: Option<Entity> = None;

                world.exec(|ui_finder: UiFinder<'_>| {
                    exit_button = ui_finder.find("exit");
                });

                if Some(target) == exit_button {
                    world.delete_all();
                    return Trans::Switch(Box::new(MainMenuState::new(
                        self.texture_loader.clone(),
                    )));
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
}
