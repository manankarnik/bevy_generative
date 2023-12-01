use crate::noise_map::{Function, FunctionName, Method, NoiseMap};
use noise::{BasicMulti, Billow, Fbm, HybridMulti, RidgedMulti};
use noise::{MultiFractal, NoiseFn, Seedable};
use noise::{OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley};

pub fn generate_noise_map(noise_map: &NoiseMap) -> Vec<Vec<f64>> {
    if let Some(function_name) = &noise_map.function.name {
        let generate_noise_map = match function_name {
            FunctionName::BasicMulti => match noise_map.method {
                Method::OpenSimplex => generate_fractal_noise::<BasicMulti<OpenSimplex>>,
                Method::Perlin => generate_fractal_noise::<BasicMulti<Perlin>>,
                Method::PerlinSurflet => generate_fractal_noise::<BasicMulti<PerlinSurflet>>,
                Method::Simplex => generate_fractal_noise::<BasicMulti<Simplex>>,
                Method::SuperSimplex => generate_fractal_noise::<BasicMulti<SuperSimplex>>,
                Method::Value => generate_fractal_noise::<BasicMulti<Value>>,
                Method::Worley => generate_fractal_noise::<BasicMulti<Worley>>,
            },
            FunctionName::Billow => match noise_map.method {
                Method::OpenSimplex => generate_fractal_noise::<Billow<OpenSimplex>>,
                Method::Perlin => generate_fractal_noise::<Billow<Perlin>>,
                Method::PerlinSurflet => generate_fractal_noise::<Billow<PerlinSurflet>>,
                Method::Simplex => generate_fractal_noise::<Billow<Simplex>>,
                Method::SuperSimplex => generate_fractal_noise::<Billow<SuperSimplex>>,
                Method::Value => generate_fractal_noise::<Billow<Value>>,
                Method::Worley => generate_fractal_noise::<Billow<Worley>>,
            },
            FunctionName::Fbm => match noise_map.method {
                Method::OpenSimplex => generate_fractal_noise::<Fbm<OpenSimplex>>,
                Method::Perlin => generate_fractal_noise::<Fbm<Perlin>>,
                Method::PerlinSurflet => generate_fractal_noise::<Fbm<PerlinSurflet>>,
                Method::Simplex => generate_fractal_noise::<Fbm<Simplex>>,
                Method::SuperSimplex => generate_fractal_noise::<Fbm<SuperSimplex>>,
                Method::Value => generate_fractal_noise::<Fbm<Value>>,
                Method::Worley => generate_fractal_noise::<Fbm<Worley>>,
            },
            FunctionName::HybridMulti => match noise_map.method {
                Method::OpenSimplex => generate_fractal_noise::<HybridMulti<OpenSimplex>>,
                Method::Perlin => generate_fractal_noise::<HybridMulti<Perlin>>,
                Method::PerlinSurflet => generate_fractal_noise::<HybridMulti<PerlinSurflet>>,
                Method::Simplex => generate_fractal_noise::<HybridMulti<Simplex>>,
                Method::SuperSimplex => generate_fractal_noise::<HybridMulti<SuperSimplex>>,
                Method::Value => generate_fractal_noise::<HybridMulti<Value>>,
                Method::Worley => generate_fractal_noise::<HybridMulti<Worley>>,
            },
            FunctionName::RidgedMulti => match noise_map.method {
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
            noise_map.size,
            noise_map.seed,
            noise_map.scale,
            noise_map.offset,
            &noise_map.function,
        )
    } else {
        let generate_noise_map = match noise_map.method {
            Method::OpenSimplex => generate_noise::<OpenSimplex>,
            Method::Perlin => generate_noise::<Perlin>,
            Method::PerlinSurflet => generate_noise::<PerlinSurflet>,
            Method::Simplex => generate_noise::<Simplex>,
            Method::SuperSimplex => generate_noise::<SuperSimplex>,
            Method::Value => generate_noise::<Value>,
            Method::Worley => generate_noise::<Worley>,
        };
        generate_noise_map(
            noise_map.size,
            noise_map.seed,
            noise_map.scale,
            noise_map.offset,
        )
    }
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
    for i in 0..(size[0] + 1) {
        let mut row: Vec<f64> = Vec::with_capacity(size[1] as usize);
        for j in 0..(size[1] + 1) {
            let x = f64::from(i as i32 - (size[0] / 2) as i32) / scale + f64::from(offset[0]);
            let y = f64::from(j as i32 - (size[1] / 2) as i32) / scale + f64::from(offset[1]);
            let value = (noise.get([x, y]) + 1.0) / 2.0 * 100.0;
            row.push(value);
        }
        noise_vector.push(row);
    }
    noise_vector
}
