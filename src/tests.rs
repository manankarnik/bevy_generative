#[cfg(test)]
mod tests {
    use crate::noise::*;

    #[test]
    fn test_generate_noise_map() {
        let noise = Noise {
            size: [100, 100],
            seed: 123,
            scale: 0.1,
            offset: [0.0, 0.0],
            method: Method::Perlin,
            function: Function::default(),
            regions: vec![],
            gradient: Gradient::default(),
            base_color: [255, 255, 255, 255],
        };
        let noise_map = generate_noise_map(&noise);
        assert_eq!(noise_map.len(), 100);
        assert_eq!(noise_map[0].len(), 100);
    }

    #[test]
    fn test_generate_noise() {
        let noise_map = generate_noise::<noise::Perlin>([100, 100], 123, 0.1, [0.0, 0.0]);
        assert_eq!(noise_map.len(), 100);
        assert_eq!(noise_map[0].len(), 100);
    }

    #[test]
    fn test_generate_fractal_noise() {
        let function = Function::default();
        let noise_map = generate_fractal_noise::<noise::Fbm<noise::Perlin>>(
            [100, 100],
            123,
            0.1,
            [0.0, 0.0],
            &function,
        );
        assert_eq!(noise_map.len(), 100);
        assert_eq!(noise_map[0].len(), 100);
    }

    #[test]
    fn test_generate_noise_vector() {
        let noise_fn = noise::Perlin::new(0);
        let noise_map = generate_noise_vector(noise_fn, [100, 100], 0.1, [0.0, 0.0]);
        assert_eq!(noise_map.len(), 100);
        assert_eq!(noise_map[0].len(), 100);
    }

    #[test]
    fn test_get_noise_at_point_3d() {
        let noise_value = get_noise_at_point_3d(
            [1.0, 2.0, 3.0],
            123,
            0.1,
            [0.0, 0.0, 0.0],
            &Method::Perlin,
            &Function::default(),
        );
        assert!(noise_value >= -1.0 && noise_value <= 1.0);
    }

    #[test]
    fn test_fractal_noise_at_point_3d() {
        let noise_value = fractal_noise_at_point_3d::<noise::Fbm<noise::Perlin>>(
            [1.0, 2.0, 3.0],
            123,
            0.1,
            [0.0, 0.0, 0.0],
            &Function::default(),
        );
        assert!(noise_value >= -1.0 && noise_value <= 1.0);
    }

    #[test]
    fn test_noise_at_point_3d() {
        let noise_value = noise_at_point_3d::<noise::Perlin>(
            [1.0, 2.0, 3.0],
            123,
            0.1,
            [0.0, 0.0, 0.0],
        );
        assert!(noise_value >= -1.0 && noise_value <= 1.0);
    }
}
