use amethyst::{
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab,
};

pub mod main_menu_state;
pub use self::main_menu_state::*;

pub mod game_state;
pub use self::game_state::*;

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;
// resource
//pub struct GameInfo {}

// impl Default for GameInfo {
//     fn default() -> Self {
//         Self {
//             round_number: 0,
//             actions: Vec::new(),
//         }
//     }
// }
