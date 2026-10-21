# Minigotchi
This is a very small desktop [Tamagotchi](https://en.wikipedia.org/wiki/Tamagotchi) built using the
[macroquad](https://macroquad.rs/) crate. Your pet starts off as an egg but hatches quickly. In
order to take care of your pet you should interact with it on a regular basis by playing with it and
giving it food.

![minigotchi](./minigotchi-ss.png)

> This `README.md` does not explain the game's mechanics. To learn about those, read: ["How to play"](HOWTOPLAY.md).


## Description

Minigotchi is a compact virtual pet intended to run on the desktop. It runs in a very small window and
can run as a standalone executable since all sprites are embedded in the software. The basic idea of the
game is to take care of a virtual pet by interacting with it once every now and again. The game stores its
state to disk as a single `minigotchi.sav` file, and uses **system time** for keeping track of the passage of time.


## Features

- Creature lifecycle
- Multiple different creatures to pick
- Feed, play and care actions
- Creature personalities
- Ability to gain *trust* from your pet
- In-game currency
- Multiple customization items
- Animated actions and simple movement behaviors
- Persistent save file written next to the executable (`minigotchi.sav`)


## Build

To build this project you'll only need to have `cargo`, `rustc` and a C-compiler like `gcc` or `clang` installed.
Once installed, a release build can be executed by running the following cargo command:

```bash
cargo build --release
```

This produces an optimized binary found at the following path relative to the project root directory:
`target/release/minigotchi`.


## Run

You can easily run the debug version of the executable by using `cargo` as well. The following command builds and runs
the debug version of the software.

```bash
cargo run
```

The release build can also be run directly in this fashion:

```bash
cargo run --release
```


## Save file

When the game saves progress to disk it creates the `minigotchi.sav` file next to the running executable. It serializes
the data from the game's state that should be persistent and writes it to the file as JSON. However,
in the **release build** of the software the JSON data is also inverted on the binary level to make
cheating a little harder.

For more details, see: [src/save_management.rs](src/save_management.rs#L1-L40).
