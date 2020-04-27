use amethyst::{
    core::ecs::{prelude::Entity, WorldExt},
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::states;
use crate::texture_loader::*;

pub struct MainMenuState {
    texture_loader: TextureLoader,
}

impl MainMenuState {
    pub fn new(texture_loader: TextureLoader) -> Self {
        Self {
            texture_loader: TextureLoader::new(),
        }
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        self.texture_loader.init(world);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("prefab/main_menu_ui.ron", ());
        });
    }

    fn update(&mut self, _state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
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
                let mut join_button: Option<Entity> = None;

                world.exec(|ui_finder: UiFinder<'_>| {
                    join_button = ui_finder.find("join");
                });

                if Some(target) == join_button {
                    world.delete_all();
                    return Trans::Switch(Box::new(states::GameState::new(
                        self.texture_loader.clone(),
                    )));
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }
}
