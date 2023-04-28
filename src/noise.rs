use noise::{NoiseFn, Perlin};

pub fn generate_noise(size: [u16; 2]) -> Vec<Vec<f64>> {
    let mut noise_space: Vec<Vec<f64>> = Vec::with_capacity(size[0] as usize);
    let noise = Perlin::new(93_493_840);
    let scale = 0.04;
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
