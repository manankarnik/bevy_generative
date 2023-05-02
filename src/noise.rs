use crate::noise_map::Method;
use noise::{NoiseFn, Seedable};
use noise::{OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex, Value, Worley};

pub fn generate_noise_by_method(
    method: &Method,
    size: [u32; 2],
    seed: u32,
    scale: f64,
) -> Vec<Vec<f64>> {
    let generate_noise_by_method: fn([u32; 2], u32, f64) -> Vec<Vec<f64>> = match method {
        Method::OpenSimplex => generate_noise::<OpenSimplex>,
        Method::Perlin => generate_noise::<Perlin>,
        Method::PerlinSurflet => generate_noise::<PerlinSurflet>,
        Method::Simplex => generate_noise::<Simplex>,
        Method::SuperSimplex => generate_noise::<SuperSimplex>,
        Method::Value => generate_noise::<Value>,
        Method::Worley => generate_noise::<Worley>,
    };
    generate_noise_by_method(size, seed, scale)
}

fn generate_noise<T>(size: [u32; 2], seed: u32, scale: f64) -> Vec<Vec<f64>>
where
    T: Default + Seedable + NoiseFn<f64, 2>,
{
    let mut noise_space: Vec<Vec<f64>> = Vec::with_capacity(size[0] as usize);
    let mut noise = T::default();
    noise = noise.set_seed(seed);
    for i in 0..size[0] {
        let mut row: Vec<f64> = Vec::with_capacity(size[1] as usize);
        for j in 0..size[1] {
            let x = f64::from(i).mul_add(scale, -f64::from(size[0] / 2));
            let y = f64::from(j).mul_add(scale, -f64::from(size[1] / 2));
            let value = noise.get([x, y]);
            row.push(value);
        }
        noise_space.push(row);
    }
    noise_space
}
