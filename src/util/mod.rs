mod gltf;
use gltf::{export_gltf, Output, Vertex};
#[cfg(not(target_arch = "wasm32"))]
use image::save_buffer;
use image::{codecs::png::PngEncoder, DynamicImage, ImageBuffer, ImageEncoder, Rgba};
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/src/util/save.js")]
extern "C" {
    fn save(data: &[u8], filename: &str, r#type: &str);
}

pub fn export_asset(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    {
        let mut png_buffer: Vec<u8> = vec![];
        let png_encoder = PngEncoder::new(&mut png_buffer);
        let color_type = DynamicImage::from(image_buffer.clone()).color();
        png_encoder
            .write_image(
                &image_buffer,
                image_buffer.width(),
                image_buffer.height(),
                color_type.into(),
            )
            .expect("Failed to write to png");

        #[cfg(target_arch = "wasm32")]
        save(&png_buffer, "asset.png", "image/png");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(file_path) = FileDialog::new().save_file() {
            let _ = save_buffer(
                file_path,
                &image_buffer.clone(),
                image_buffer.width(),
                image_buffer.height(),
                DynamicImage::from(image_buffer).color(),
            );
        }
    }
}

pub fn export_model(positions: &[[f32; 3]], indices: Vec<u32>, colors: &[[f32; 4]]) {
    let mut vertices: Vec<Vertex> = vec![];

    for i in indices {
        vertices.push(Vertex {
            position: positions[i as usize],
            color: [
                colors[i as usize][0],
                colors[i as usize][1],
                colors[i as usize][2],
            ],
        });
    }
    export_gltf(Output::Binary, vertices);
}
