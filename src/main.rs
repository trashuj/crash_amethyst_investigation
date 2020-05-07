extern crate serde;
//extern crate serde_derive;
extern crate serde_json;

use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod grid;
mod states;
mod systems;
mod texture_loader;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let bindings_config = config_dir.join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_system_desc(
            PrefabLoaderSystemDesc::<states::MyPrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            systems::CameraSystem::new(),
            "camera_system",
            &["input_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.0]),
                )
                //.with_plugin(RenderPbr3D::default())
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )?;

    //let mut game = Application::new(assets_dir, states::MainMenuState::new(), game_data)?;
    let mut game = Application::new(assets_dir, states::GameState::new(), game_data)?;
    game.run();

    Ok(())
}
