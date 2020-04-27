# crash investigation
This is my 1st project using rust and amethyst.
As a conveniance i tried to implement a texture loader that would store all the texture handles.
But, when running the game and going back and forth between the game state and the main menu state, it appears that the textures are.. moving around in memory???
the 1st run is always correct, so it's the pattern that you should see every time.

After 2 or three back and forth, the textures get mixed up,
Eventually, it might crash :(

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
