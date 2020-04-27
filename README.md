# crash investigation
This is my 1st project using rust and amethyst.
As a conveniance i tried to implement a texture loader that would store all the texture handles.
But, when running the game and going back and forth between the game state and the main menu state, it appears that the textures are.. moving around in memory???
the 1st run is always correct, so it's the pattern that you should see every time.

After 2 or three back and forth, the textures get mixed up,
Eventually, it might crash :(

Also note that you'll get 2 warnings:

    warning: unused variable: `texture_loader`
    --> src\states\main_menu_state.rs:16:16
    |
    16 |     pub fn new(texture_loader: TextureLoader) -> Self {
    |                ^^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_texture_loader`
    |
    = note: `#[warn(unused_variables)]` on by default

    warning: unused variable: `texture_loader`
    --> src\states\game_state.rs:57:16
    |
    57 |     pub fn new(texture_loader: TextureLoader) -> Self {
    |   

This is the "fix" I came up with. Basically if you pass the texture loader from states to states, then it's never reloaded and everything works fine.
It's definitely a work around, but eventually i dont want to load all the assets if i dont need them..

If you could run this app and help me figure out what's going on, I would really appreciate.
thanks a lot in advance!

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
