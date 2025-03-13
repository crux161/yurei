use image::{ImageBuffer, Rgb};
use std::fs;
use std::path::Path;

struct ImageProcessor {
    file_name: String,
}

impl ImageProcessor {
    fn new(file_name: String) -> ImageProcessor {
        ImageProcessor { file_name }
    }

    fn render_and_save(&self, b: &[u8]) -> Result<(), image::ImageError> {
        // Use self.file_name here
        let img = create_image(b, 1024)?;
        let output = format!("{}.png", self.file_name);
        img.save(&output)?;
        println!("Image saved to {}, {}", output, self.file_name);
        Ok(())
    }
}

fn read_flash_image(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(Path::new(file_path))
}

fn render_plot(b: &[u8], name: &str) -> Result<(), image::ImageError> {
    const BINLINE_LENGTH: usize = 1024;
    let pre_pad = (b.len() as f64 / BINLINE_LENGTH as f64).ceil() as usize * BINLINE_LENGTH;
    let mut post_pad = vec![0; pre_pad];
    post_pad[..b.len()].copy_from_slice(b);

    let img = create_image(&post_pad, BINLINE_LENGTH)?;
    //draw_plot(&img, name)?;
    Ok(())
}
/*
fn create_image(b: &[u8], binline_length: usize) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, image::ImageError> {
    let width = binline_length as u32;
    let height = (b.len() as f64 / binline_length as f64).ceil() as u32;
    let mut img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let value = b[(y * width + x) as usize];
            let rgb = Rgb([value, value, value]);
            img.put_pixel(x, y, rgb);
        }
    }
    Ok(img)
}*/
fn create_image(b: &[u8], binline_length: usize) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, image::ImageError> {
    let width = binline_length as u32;
    let height = (b.len() as u32 + width -1) / width ;
    let mut img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            if index < b.len(){
                let value = b[index];
                let rgb = Rgb([value, value, value]);
                img.put_pixel(x, y, rgb);
            }
        }
    }
    Ok(img)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: yurei <file_path>");
        std::process::exit(1);
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);
    if let Some(file_name_osstr) = file_path.file_name() {
        if let Some(file_name) = file_name_osstr.to_str() {
            let processor = ImageProcessor::new(file_name.to_string());
            let b = read_flash_image(file_path_str)?;
            processor.render_and_save(&b)?;
            Ok(())
        } else {
            eprintln!("Error: Invalid file name.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: Could not extract file name.");
        std::process::exit(1);
    }
}
