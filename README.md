# bevy_generative

![CodeSize](https://img.shields.io/github/languages/code-size/manankarnik/bevy_generative?style=for-the-badge)
[![License](https://img.shields.io/badge/license-MIT%2FApache-red.svg?style=for-the-badge)](https://github.com/manankarnik/bevy_generative#license)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/manankarnik/bevy_generative/ci.yml?style=for-the-badge)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=for-the-badge)](https://bevyengine.org/learn/book/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_generative?style=for-the-badge)](https://crates.io/crates/bevy_generative)
[![docs.rs](https://img.shields.io/badge/docs-online-brightgreen.svg?style=for-the-badge)](https://docs.rs/bevy_generative)

`bevy_generative` is a plugin written for the [bevy engine](https://bevyengine.org/) that allows real-time procedural generation of maps, textures, terrain, planets and more!

## Features

- Allows procedural generation of assets which can be directly integrated in your bevy game
- Handles real-time updating of image and mesh data as configuration for the asset changes
- Builds on native as well as wasm targets
- Allows saving generated assets (uses `rfd` for native, javascript blob for wasm)
- Serializes and deserializes components using `serde`

## Installation

Add `bevy_generative` to your rust project

```sh
cargo add bevy_generative
```

## Examples

### Maps and Textures

```rust
use bevy::prelude::*;
use bevy_generative::map::{MapBundle, MapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MapBundle::default());
}

```

### Terrain

```rust
use bevy::prelude::*;
use bevy_generative::terrain::{TerrainBundle, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(TerrainBundle::default());
}

```

### Planets

```rust
use bevy::prelude::*;
use bevy_generative::planet::{PlanetBundle, PlanetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PlanetBundle::default());
}

```

## Bevy Compatibility

| bevy | bevy_generative |
| ---- | --------------- |
| 0.12 | 0.1, main       |

## Contributing

Not accepting pull requests at this time. Issues, feature requests and bug reports are appreciated.

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
