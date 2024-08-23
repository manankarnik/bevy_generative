# Changelog

### [v0.3.1](https://github.com/manankarnik/bevy_generative/compare/v0.3.0...v0.3.1) (2024-08-23)

## [v0.3.0](https://github.com/manankarnik/bevy_generative/compare/v0.2.0...v0.3.0) (2024-08-23)

## [v0.2.0](https://github.com/manankarnik/bevy_generative/compare/v0.1.0...v0.2.0) (2024-03-08)

### Features

* add image size
([fbd760b](https://github.com/manankarnik/bevy_generative/commit/fbd760b3e579f6f3b6cff806a3d96c6f27d10b2f))
* derive serde for components
([466fd57](https://github.com/manankarnik/bevy_generative/commit/466fd5731b49af0408f42281e45828ab4c45eba5))

### Fixes

* serde default all structs
([232b697](https://github.com/manankarnik/bevy_generative/commit/232b69782e5567b3e0bab4c09554c984ccbe88cd))
* wasm-bindgen called on non-wasm targets
([3aa437e](https://github.com/manankarnik/bevy_generative/commit/3aa437e72e62a0d42c4c93020a22daed86018f7b))

## v0.1.0 (2024-01-07)

### Features

* **planet:** add elevation and colors
([6efd336](https://github.com/manankarnik/bevy_generative/commit/6efd336a116aa79916eec57ecec5238dfad511cb))
* **terrain:** add height exponent and resolution
([09793f2](https://github.com/manankarnik/bevy_generative/commit/09793f21315cfb7bc8660ee3b7b03862a8783ead))
* add planet generation
([6444ac1](https://github.com/manankarnik/bevy_generative/commit/6444ac199c85afc7e09250f8b8144d824e52e57c))
* add terrain sea level and height exponent
([0979da0](https://github.com/manankarnik/bevy_generative/commit/0979da0806bea5e2e9b95ddc049c8adb8dd698e6))
* add export for assets
([386ea78](https://github.com/manankarnik/bevy_generative/commit/386ea78be2b218f118ae0c4b2665617d789d991e))
* add trunk and wasm
([37b294b](https://github.com/manankarnik/bevy_generative/commit/37b294b567d27f8995c6a7a116b5035736287fe2))
* add wireframe, bloom, export for terrain
([dc295f0](https://github.com/manankarnik/bevy_generative/commit/dc295f07ca846cd0f94862c073c430c96e4457e9))
* add wireframe rendering for terrain
([0c5ed53](https://github.com/manankarnik/bevy_generative/commit/0c5ed531b2f0aed98654326bc6ea8a58f7b26b3e))
* add terrain generation
([b732d5f](https://github.com/manankarnik/bevy_generative/commit/b732d5f252b949249084620b4e2eda906ceafad4))
* add alpha and blending
([8b1d30d](https://github.com/manankarnik/bevy_generative/commit/8b1d30d4deb671092e6068dc95f704c5071744c5))
* add color gradients
([19a6a59](https://github.com/manankarnik/bevy_generative/commit/19a6a59de3004ee0de1df89e62d8f69bd7e2e99a))
* offset ui
([2d6d68f](https://github.com/manankarnik/bevy_generative/commit/2d6d68fe3a659babbdbfd2664d3fbc6e7c77ad25))
* anti-aliasing and method/function selection
([b251d9f](https://github.com/manankarnik/bevy_generative/commit/b251d9f057ef319f25a6677af4710dcfada0c539))
* add regions in egui
([d5ca581](https://github.com/manankarnik/bevy_generative/commit/d5ca581fef557f7767b8ed4ede4c998e7d1ee13e))
* add threshold region and update scale
([e5c5a9b](https://github.com/manankarnik/bevy_generative/commit/e5c5a9b54fa1fecea2cc2110621be42910313f7b))
* add noise functions
([b9939e8](https://github.com/manankarnik/bevy_generative/commit/b9939e8b80c9f33734445eac78f5c8576e567d58))
* add regions
([fc13d1f](https://github.com/manankarnik/bevy_generative/commit/fc13d1fdda04d5aad17381b3ca0cb2e22bae2d05))
* add noise map bundle
([41a4afd](https://github.com/manankarnik/bevy_generative/commit/41a4afd570ac8ff18709dced70a3c9c06acae487))
* add offset to noise map
([2316311](https://github.com/manankarnik/bevy_generative/commit/2316311dfdbb98417ef568e51048996de13fe0db))
* add noise map marker component
([6a93f69](https://github.com/manankarnik/bevy_generative/commit/6a93f6911b4af0a11ec1a6e3c5e2882bf6e049c0))
* add noise map config
([e6774f5](https://github.com/manankarnik/bevy_generative/commit/e6774f5eb7b1a8eede32de50301e0a6a5773df60))
* add noise map generation
([f723ff8](https://github.com/manankarnik/bevy_generative/commit/f723ff8007ec1df55ed43e8d4eb2d95d64082048))

### Fixes

* change extras type
([dbc14fa](https://github.com/manankarnik/bevy_generative/commit/dbc14fa20a0b495ef9b9500058729c7001c94197))
* add save_buffer import
([2a096a1](https://github.com/manankarnik/bevy_generative/commit/2a096a1d1e460ab60bb99f68c5a72664f1d7ef8b))
* **noise:** clamp 3d noise
([fa0b32c](https://github.com/manankarnik/bevy_generative/commit/fa0b32cc75ad283d3eae82b35eacdd25280443b7))
* **planet:** tweak noise and height values
([33ce5c7](https://github.com/manankarnik/bevy_generative/commit/33ce5c7ff64064c4633239d742dc8ba64cb989e3))
* private field size
([1ff47e9](https://github.com/manankarnik/bevy_generative/commit/1ff47e94cc138044d6cd434be88ae35e4a1d8532))
* remove imports
([7199ce3](https://github.com/manankarnik/bevy_generative/commit/7199ce395d5feb9ae9154e05cda997fc28c36aa1))
* hard coded size causing panic
([18702f7](https://github.com/manankarnik/bevy_generative/commit/18702f79662b00574f600fde08c2901f15183d09))
