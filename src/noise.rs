use core::fmt;

use bevy::prelude::{Handle, Image};
use noise::{BasicMulti, Billow, Fbm, HybridMulti, RidgedMulti};
use noise::{MultiFractal, NoiseFn, Seedable};
use noise::{OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley};
use serde::{Deserialize, Serialize};

/// 2D noise method used to generate noise map
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    /// Open Simplex noise
    OpenSimplex,
    /// Perlin noise
    Perlin,
    /// Perlin Surflet noise
    PerlinSurflet,
    /// Simplex noise
    Simplex,
    /// Super Simplex noise
    SuperSimplex,
    /// Value noise
    Value,
    /// Worley noise
    Worley,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenSimplex => write!(f, "Open Simplex"),
            Self::Perlin => write!(f, "Perlin"),
            Self::PerlinSurflet => write!(f, "Perlin Surflet"),
            Self::Simplex => write!(f, "Simplex"),
            Self::SuperSimplex => write!(f, "Super Simplex"),
            Self::Value => write!(f, "Value"),
            Self::Worley => write!(f, "Worley"),
        }
    }
}

/// Fractal function that should be applied on the noise values
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionName {
    /// See [`BasicMulti`](https://docs.rs/noise/latest/noise/struct.BasicMulti.html)
    BasicMulti,
    /// See [`Billow`](https://docs.rs/noise/latest/noise/struct.Billow.html)
    Billow,
    /// See [`Fbm`](https://docs.rs/noise/latest/noise/struct.Fbm.html)
    Fbm,
    /// See [`HybridMulti`](https://docs.rs/noise/latest/noise/struct.HybridMulti.html)
    HybridMulti,
    /// See [`RidgedMulti`](https://docs.rs/noise/latest/noise/struct.RidgedMulti.html)
    RidgedMulti,
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BasicMulti => write!(f, "Basic Multi"),
            Self::Billow => write!(f, "Billow"),
            Self::Fbm => write!(f, "FBM"),
            Self::HybridMulti => write!(f, "Hybrid Multi"),
            Self::RidgedMulti => write!(f, "Ridged Multi"),
        }
    }
}

/// Fractal function configuration
#[derive(Serialize, Deserialize)]
pub struct Function {
    /// Name of the function
    pub name: Option<FunctionName>,
    /// See [`Octaves`](https://docs.rs/noise/latest/noise/struct.BasicMulti.html#structfield.octaves)
    pub octaves: usize,
    /// See [`Frequency`](https://docs.rs/noise/latest/noise/struct.BasicMulti.html#structfield.octaves)
    pub frequency: f64,
    /// See [`Lacunarity`](https://docs.rs/noise/latest/noise/struct.BasicMulti.html#structfield.octaves)
    pub lacunarity: f64,
    /// See [`Persistance`](https://docs.rs/noise/latest/noise/struct.BasicMulti.html#structfield.octaves)
    pub persistence: f64,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: Some(FunctionName::Fbm),
            octaves: noise::Fbm::<noise::Perlin>::DEFAULT_OCTAVE_COUNT,
            frequency: noise::Fbm::<noise::Perlin>::DEFAULT_FREQUENCY,
            lacunarity: noise::Fbm::<noise::Perlin>::DEFAULT_LACUNARITY,
            persistence: noise::Fbm::<noise::Perlin>::DEFAULT_PERSISTENCE,
        }
    }
}

/// Region based on height
#[derive(Serialize, Deserialize)]
pub struct Region {
    /// Label of the region
    pub label: String,
    /// Percentage below which the region should render
    pub position: f64,
    /// Color representing the region
    pub color: [u8; 4],
}

impl Default for Region {
    fn default() -> Self {
        Self {
            label: String::new(),
            position: 0.0,
            color: [0, 0, 0, 255],
        }
    }
}

/// Gradient used to map color values
#[derive(Serialize, Deserialize)]
pub struct Gradient {
    /// Image handle of gradient
    #[serde(skip_serializing, skip_deserializing)]
    pub image: Handle<Image>,
    /// Size of gradient
    pub size: [u32; 2],
    /// Segments in gradient (0 for infinite)
    pub segments: usize,
    /// Smoothness between gradient borders
    pub smoothness: f64,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            image: Handle::default(),
            size: [250, 50],
            segments: 3,
            smoothness: 0.0,
        }
    }
}

/// Noise configuration
#[derive(Serialize, Deserialize)]
pub struct Noise {
    pub(crate) size: [u32; 2],
    /// Seed of the noise
    pub seed: u32,
    /// Scale of the noise
    pub scale: f64,
    /// Offset of the noise
    pub offset: [f64; 2],
    /// Method used to generate noise
    pub method: Method,
    /// Function used to generate noise
    pub function: Function,
    /// Vector of regions
    pub regions: Vec<Region>,
    /// Gradient determines how the noise values are mapped to colors
    pub gradient: Gradient,
    /// Base color of the gradient.
    /// If gradient has transparency, base color will be blended with the gradient
    pub base_color: [u8; 4],
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            size: [500; 2],
            seed: 0,
            scale: 50.0,
            offset: [0.0; 2],
            method: Method::Perlin,
            function: Function::default(),
            regions: vec![
                Region {
                    label: "Sand".to_string(),
                    color: [242, 241, 199, 255],
                    position: 0.0,
                },
                Region {
                    label: "Grass".to_string(),
                    color: [24, 148, 67, 255],
                    position: 50.0,
                },
                Region {
                    label: "Forest".to_string(),
                    color: [10, 82, 35, 255],
                    position: 100.0,
                },
            ],
            gradient: Gradient::default(),
            base_color: [255, 255, 255, 255],
        }
    }
}

pub(crate) fn generate_noise_map(noise: &Noise) -> Vec<Vec<f64>> {
    noise.function.name.as_ref().map_or_else(
        || {
            let generate_noise_map = match noise.method {
                Method::OpenSimplex => generate_noise::<OpenSimplex>,
                Method::Perlin => generate_noise::<Perlin>,
                Method::PerlinSurflet => generate_noise::<PerlinSurflet>,
                Method::Simplex => generate_noise::<Simplex>,
                Method::SuperSimplex => generate_noise::<SuperSimplex>,
                Method::Value => generate_noise::<Value>,
                Method::Worley => generate_noise::<Worley>,
            };
            generate_noise_map(noise.size, noise.seed, noise.scale, noise.offset)
        },
        |function_name| {
            let generate_noise_map = match function_name {
                FunctionName::BasicMulti => match noise.method {
                    Method::OpenSimplex => generate_fractal_noise::<BasicMulti<OpenSimplex>>,
                    Method::Perlin => generate_fractal_noise::<BasicMulti<Perlin>>,
                    Method::PerlinSurflet => generate_fractal_noise::<BasicMulti<PerlinSurflet>>,
                    Method::Simplex => generate_fractal_noise::<BasicMulti<Simplex>>,
                    Method::SuperSimplex => generate_fractal_noise::<BasicMulti<SuperSimplex>>,
                    Method::Value => generate_fractal_noise::<BasicMulti<Value>>,
                    Method::Worley => generate_fractal_noise::<BasicMulti<Worley>>,
                },
                FunctionName::Billow => match noise.method {
                    Method::OpenSimplex => generate_fractal_noise::<Billow<OpenSimplex>>,
                    Method::Perlin => generate_fractal_noise::<Billow<Perlin>>,
                    Method::PerlinSurflet => generate_fractal_noise::<Billow<PerlinSurflet>>,
                    Method::Simplex => generate_fractal_noise::<Billow<Simplex>>,
                    Method::SuperSimplex => generate_fractal_noise::<Billow<SuperSimplex>>,
                    Method::Value => generate_fractal_noise::<Billow<Value>>,
                    Method::Worley => generate_fractal_noise::<Billow<Worley>>,
                },
                FunctionName::Fbm => match noise.method {
                    Method::OpenSimplex => generate_fractal_noise::<Fbm<OpenSimplex>>,
                    Method::Perlin => generate_fractal_noise::<Fbm<Perlin>>,
                    Method::PerlinSurflet => generate_fractal_noise::<Fbm<PerlinSurflet>>,
                    Method::Simplex => generate_fractal_noise::<Fbm<Simplex>>,
                    Method::SuperSimplex => generate_fractal_noise::<Fbm<SuperSimplex>>,
                    Method::Value => generate_fractal_noise::<Fbm<Value>>,
                    Method::Worley => generate_fractal_noise::<Fbm<Worley>>,
                },
                FunctionName::HybridMulti => match noise.method {
                    Method::OpenSimplex => generate_fractal_noise::<HybridMulti<OpenSimplex>>,
                    Method::Perlin => generate_fractal_noise::<HybridMulti<Perlin>>,
                    Method::PerlinSurflet => generate_fractal_noise::<HybridMulti<PerlinSurflet>>,
                    Method::Simplex => generate_fractal_noise::<HybridMulti<Simplex>>,
                    Method::SuperSimplex => generate_fractal_noise::<HybridMulti<SuperSimplex>>,
                    Method::Value => generate_fractal_noise::<HybridMulti<Value>>,
                    Method::Worley => generate_fractal_noise::<HybridMulti<Worley>>,
                },
                FunctionName::RidgedMulti => match noise.method {
                    Method::OpenSimplex => generate_fractal_noise::<RidgedMulti<OpenSimplex>>,
                    Method::Perlin => generate_fractal_noise::<RidgedMulti<Perlin>>,
                    Method::PerlinSurflet => generate_fractal_noise::<RidgedMulti<PerlinSurflet>>,
                    Method::Simplex => generate_fractal_noise::<RidgedMulti<Simplex>>,
                    Method::SuperSimplex => generate_fractal_noise::<RidgedMulti<SuperSimplex>>,
                    Method::Value => generate_fractal_noise::<RidgedMulti<Value>>,
                    Method::Worley => generate_fractal_noise::<RidgedMulti<Worley>>,
                },
            };
            generate_noise_map(
                noise.size,
                noise.seed,
                noise.scale,
                noise.offset,
                &noise.function,
            )
        },
    )
}

fn generate_noise<T>(size: [u32; 2], seed: u32, scale: f64, offset: [f64; 2]) -> Vec<Vec<f64>>
where
    T: Default + Seedable + NoiseFn<f64, 2>,
{
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    generate_noise_vector(noise, size, scale, offset)
}

fn generate_fractal_noise<T>(
    size: [u32; 2],
    seed: u32,
    scale: f64,
    offset: [f64; 2],
    function: &Function,
) -> Vec<Vec<f64>>
where
    T: Default + Seedable + NoiseFn<f64, 2> + MultiFractal,
{
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    noise = noise.set_octaves(function.octaves);
    noise = noise.set_frequency(function.frequency);
    noise = noise.set_lacunarity(function.lacunarity);
    noise = noise.set_persistence(function.persistence);
    generate_noise_vector(noise, size, scale, offset)
}

fn generate_noise_vector(
    noise: impl NoiseFn<f64, 2>,
    size: [u32; 2],
    scale: f64,
    offset: [f64; 2],
) -> Vec<Vec<f64>> {
    let mut noise_vector: Vec<Vec<f64>> = Vec::with_capacity(size[0] as usize);
    let noise = noise::Clamp::new(noise).set_bounds(-1.0, 1.0);
    for i in 0..=size[0] {
        let mut row: Vec<f64> = Vec::with_capacity(size[1] as usize);
        for j in 0..=size[1] {
            let x = f64::from(i as i32 - (size[0] / 2) as i32) / scale + offset[0];
            let y = f64::from(j as i32 - (size[1] / 2) as i32) / scale + offset[1];
            let value = (noise.get([x, y]) + 1.0) / 2.0 * 100.0;
            row.push(value);
        }
        noise_vector.push(row);
    }
    noise_vector
}

pub(crate) fn get_noise_at_point_3d(
    point: [f64; 3],
    seed: u32,
    scale: f64,
    offset: [f64; 3],
    method: &Method,
    function: &Function,
) -> f64 {
    function.name.as_ref().map_or_else(
        || {
            let noise_at_point_3d = match method {
                Method::OpenSimplex => noise_at_point_3d::<OpenSimplex>,
                Method::Perlin => noise_at_point_3d::<Perlin>,
                Method::PerlinSurflet => noise_at_point_3d::<PerlinSurflet>,
                Method::Simplex => noise_at_point_3d::<Simplex>,
                Method::SuperSimplex => noise_at_point_3d::<SuperSimplex>,
                Method::Value => noise_at_point_3d::<Value>,
                Method::Worley => noise_at_point_3d::<Worley>,
            };
            noise_at_point_3d(point, seed, scale, offset)
        },
        |function_name| {
            let noise_at_point_3d = match function_name {
                FunctionName::BasicMulti => match method {
                    Method::OpenSimplex => fractal_noise_at_point_3d::<BasicMulti<OpenSimplex>>,
                    Method::Perlin => fractal_noise_at_point_3d::<BasicMulti<Perlin>>,
                    Method::PerlinSurflet => fractal_noise_at_point_3d::<BasicMulti<PerlinSurflet>>,
                    Method::Simplex => fractal_noise_at_point_3d::<BasicMulti<Simplex>>,
                    Method::SuperSimplex => fractal_noise_at_point_3d::<BasicMulti<SuperSimplex>>,
                    Method::Value => fractal_noise_at_point_3d::<BasicMulti<Value>>,
                    Method::Worley => fractal_noise_at_point_3d::<BasicMulti<Worley>>,
                },
                FunctionName::Billow => match method {
                    Method::OpenSimplex => fractal_noise_at_point_3d::<Billow<OpenSimplex>>,
                    Method::Perlin => fractal_noise_at_point_3d::<Billow<Perlin>>,
                    Method::PerlinSurflet => fractal_noise_at_point_3d::<Billow<PerlinSurflet>>,
                    Method::Simplex => fractal_noise_at_point_3d::<Billow<Simplex>>,
                    Method::SuperSimplex => fractal_noise_at_point_3d::<Billow<SuperSimplex>>,
                    Method::Value => fractal_noise_at_point_3d::<Billow<Value>>,
                    Method::Worley => fractal_noise_at_point_3d::<Billow<Worley>>,
                },
                FunctionName::Fbm => match method {
                    Method::OpenSimplex => fractal_noise_at_point_3d::<Fbm<OpenSimplex>>,
                    Method::Perlin => fractal_noise_at_point_3d::<Fbm<Perlin>>,
                    Method::PerlinSurflet => fractal_noise_at_point_3d::<Fbm<PerlinSurflet>>,
                    Method::Simplex => fractal_noise_at_point_3d::<Fbm<Simplex>>,
                    Method::SuperSimplex => fractal_noise_at_point_3d::<Fbm<SuperSimplex>>,
                    Method::Value => fractal_noise_at_point_3d::<Fbm<Value>>,
                    Method::Worley => fractal_noise_at_point_3d::<Fbm<Worley>>,
                },
                FunctionName::HybridMulti => match method {
                    Method::OpenSimplex => fractal_noise_at_point_3d::<HybridMulti<OpenSimplex>>,
                    Method::Perlin => fractal_noise_at_point_3d::<HybridMulti<Perlin>>,
                    Method::PerlinSurflet => {
                        fractal_noise_at_point_3d::<HybridMulti<PerlinSurflet>>
                    }
                    Method::Simplex => fractal_noise_at_point_3d::<HybridMulti<Simplex>>,
                    Method::SuperSimplex => fractal_noise_at_point_3d::<HybridMulti<SuperSimplex>>,
                    Method::Value => fractal_noise_at_point_3d::<HybridMulti<Value>>,
                    Method::Worley => fractal_noise_at_point_3d::<HybridMulti<Worley>>,
                },
                FunctionName::RidgedMulti => match method {
                    Method::OpenSimplex => fractal_noise_at_point_3d::<RidgedMulti<OpenSimplex>>,
                    Method::Perlin => fractal_noise_at_point_3d::<RidgedMulti<Perlin>>,
                    Method::PerlinSurflet => {
                        fractal_noise_at_point_3d::<RidgedMulti<PerlinSurflet>>
                    }
                    Method::Simplex => fractal_noise_at_point_3d::<RidgedMulti<Simplex>>,
                    Method::SuperSimplex => fractal_noise_at_point_3d::<RidgedMulti<SuperSimplex>>,
                    Method::Value => fractal_noise_at_point_3d::<RidgedMulti<Value>>,
                    Method::Worley => fractal_noise_at_point_3d::<RidgedMulti<Worley>>,
                },
            };
            noise_at_point_3d(point, seed, scale, offset, function)
        },
    )
}

fn fractal_noise_at_point_3d<T>(
    point: [f64; 3],
    seed: u32,
    scale: f64,
    offset: [f64; 3],
    function: &Function,
) -> f64
where
    T: Default + Seedable + NoiseFn<f64, 3> + MultiFractal,
{
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    noise = noise.set_octaves(function.octaves);
    noise = noise.set_frequency(function.frequency);
    noise = noise.set_lacunarity(function.lacunarity);
    noise = noise.set_persistence(function.persistence);
    let x = point[0] / scale + offset[0];
    let y = point[1] / scale + offset[1];
    let z = point[2] / scale + offset[2];
    let noise = noise::Clamp::new(noise).set_bounds(-1.0, 1.0);
    noise.get([x, y, z])
}

fn noise_at_point_3d<T>(point: [f64; 3], seed: u32, scale: f64, offset: [f64; 3]) -> f64
where
    T: Default + Seedable + NoiseFn<f64, 3>,
{
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    let x = point[0] / scale + offset[0];
    let y = point[1] / scale + offset[1];
    let z = point[2] / scale + offset[2];
    let noise = noise::Clamp::new(noise).set_bounds(-1.0, 1.0);
    noise.get([x, y, z])
}
