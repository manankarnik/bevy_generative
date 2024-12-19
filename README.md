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

## Showcase

Check out [Procedra](https://procedra.netlify.app) [[Source](https://github.com/manankarnik/procedra)], a procedural generation web application that allows you to experiment with all the parameters and generate assets in real-time!

## Installation

Add `bevy_generative` to your rust project

```sh
cargo add bevy_generative
```

## Examples

Examples are provided in the [examples](./examples) directory. To run an example, clone this repository and invoke cargo like this:

```sh
cargo run --example map
```

## Bevy Compatibility

| bevy | bevy_generative |
| ---- | --------------- |
| 0.14 | 0.3, main       |
| 0.13 | 0.2             |
| 0.12 | 0.1             |

## Contributing

Contributions are welcome! Issues, pull requests, feature requests and bug reports are appreciated. If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch with a descriptive name.
3. Make your changes or additions.
4. Test your changes.
5. Submit a pull request with a clear description of your work.

Please ensure your code passes all CI checks and includes relevant tests if applicable. Thank you for helping improve `bevy_generative`!

Your contribution will be dual-licensed as mentioned in the License section below.

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
