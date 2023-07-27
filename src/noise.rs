use crate::noise_map::{Method, NoiseMap};
use bevy::log::info;
use noise::{Fbm, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley};
use noise::{NoiseFn, Seedable};

pub fn generate_noise_map(noise_map: &NoiseMap) -> Vec<Vec<f64>> {
    let generate_noise_map: fn([u32; 2], u32, f64, [i32; 2]) -> Vec<Vec<f64>> =
        match noise_map.method {
            Method::OpenSimplex => generate_noise::<Fbm<OpenSimplex>>,
            Method::Perlin => generate_noise::<Fbm<Perlin>>,
            Method::PerlinSurflet => generate_noise::<Fbm<PerlinSurflet>>,
            Method::Simplex => generate_noise::<Fbm<Simplex>>,
            Method::SuperSimplex => generate_noise::<Fbm<SuperSimplex>>,
            Method::Value => generate_noise::<Fbm<Value>>,
            Method::Worley => generate_noise::<Fbm<Worley>>,
        };
    generate_noise_map(
        noise_map.size,
        noise_map.seed,
        noise_map.scale,
        noise_map.offset,
    )
}

fn generate_noise<T>(size: [u32; 2], seed: u32, scale: f64, offset: [i32; 2]) -> Vec<Vec<f64>>
where
    T: Default + Seedable + NoiseFn<f64, 2>,
{
    let mut noise_space: Vec<Vec<f64>> = Vec::with_capacity(size[0] as usize);
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    for i in 0..size[0] {
        let mut row: Vec<f64> = Vec::with_capacity(size[1] as usize);
        for j in 0..size[1] {
            let x = f64::from(i as i32 + offset[0]).mul_add(scale, -f64::from(size[0] / 2));
            let y = f64::from(j as i32 + offset[1]).mul_add(scale, -f64::from(size[1] / 2));
            let value = noise.get([x, y]);
            row.push(value);
        }
        noise_space.push(row);
    }
    noise_space
}
