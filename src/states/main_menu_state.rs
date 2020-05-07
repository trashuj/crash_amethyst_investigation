use amethyst::{
    core::ecs::{prelude::Entity, WorldExt},
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::states;

pub struct MainMenuState {}

impl MainMenuState {
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

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
                    return Trans::Switch(Box::new(states::GameState::new()));
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }
}
