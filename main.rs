#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;
use colors_transform::Rgb;
use image::{self, GenericImageView};
use skia_safe::{AlphaType, Color4f, ColorType, EncodedImageFormat, ImageInfo, Paint, Rect, Surface};
use css_color_parser::Color as CssColor;
use uuid::Uuid;
use std::{env, fs, io::{self, Write}, path::PathBuf};

/// Generates a unique temporary file path
fn generate_temp_file_path() -> PathBuf {
    let uuid = Uuid::new_v4();
    PathBuf::from(format!("temp_{}.png", uuid))
}

/// Converts a PNG file to a `.bruh` format
fn png_to_bruh(path: PathBuf) -> io::Result<()> {
    let img = image::open(&path).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid image file"))?;
    let mut result = String::new();
    let mut last_line = 0;

    for pixel in img.pixels() {
        let hex_color = Rgb::from(pixel.2 .0[0] as f32, pixel.2 .0[1] as f32, pixel.2 .0[2] as f32)
        .to_css_hex_string();
        if last_line != pixel.1 {
            result.push('\n');
            last_line = pixel.1;
        }
        result.push_str(&hex_color.replace('#', ""));
    }

    let height = img.height();
    let width = img.width();

    let height_bytes = height.to_ne_bytes();
    let width_bytes = width.to_ne_bytes();

    let path_to_bruh = path.with_extension("bruh");
    let mut file = fs::File::create(&path_to_bruh)?;

    file.write_all(&width_bytes)?;
    file.write_all(&height_bytes)?;
    file.write_all(result.as_bytes())?;

    println!("Successfully converted PNG to BRUH: {:?}", path_to_bruh);
    Ok(())
}

/// Converts a `.bruh` file to a PNG and returns its dimensions
fn bruh_to_png(path: PathBuf) -> io::Result<(u32, u32, PathBuf)> {
    let contents = fs::read(&path)?;
    let (width_bytes, height_bytes) = contents.split_at(4);
    let width = u32::from_ne_bytes(width_bytes.try_into().unwrap());
    let height = u32::from_ne_bytes(height_bytes[0..4].try_into().unwrap());

    let color_data = &contents[8..];
    let sanitized_content = String::from_utf8_lossy(color_data).replace("\n", "");
    let colors: Vec<&str> = sanitized_content.as_bytes()
    .chunks(6)
    .map(std::str::from_utf8)
    .collect::<Result<_, _>>()
    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid BRUH file format"))?;

    let info = ImageInfo::new(
        (width as i32, height as i32),
                              ColorType::RGBA8888,
                              AlphaType::Opaque,
                              None,
    );
    let mut surface = Surface::new_raster(&info, None, None).unwrap();
    let canvas = surface.canvas();

    for (i, color) in colors.iter().enumerate() {
        let hex = format!("#{}", color);
        let parsed_color = hex.parse::<CssColor>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid color"))?;
        let color4f = Color4f::new(parsed_color.r as f32, parsed_color.g as f32, parsed_color.b as f32, 1.0);
        let paint = Paint::new(color4f, None);

        let x = i % width as usize;
        let y = i / width as usize;

        let rect = Rect::from_point_and_size((x as f32, y as f32), (1.0, 1.0));
        canvas.draw_rect(rect, &paint);
    }

    let image = surface.image_snapshot();
    let temp_file_path = generate_temp_file_path();
    if let Some(data) = image.encode(None, EncodedImageFormat::PNG, 100) {
        fs::write(&temp_file_path, &*data)?;
    }

    Ok((width, height, temp_file_path))
}

/// Main application
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  To compile PNG to BRUH: `cargo run compile <image.png>`");
        println!("  To preview BRUH file: `cargo run <file.bruh>`");
        return Ok(());
    }

    match args[1].as_str() {
        "compile" => {
            if args.len() < 3 {
                println!("Please provide a valid PNG file path.");
                return Ok(());
            }
            png_to_bruh(PathBuf::from(&args[2]))?;
        }
        _ => {
            let (width, height, temp_file) = bruh_to_png(PathBuf::from(&args[1]))?;
            let options = eframe::NativeOptions {
                resizable: false,
                initial_window_size: Some(egui::vec2(width as f32, height as f32)),
                ..Default::default()
            };
            if let Err(e) = eframe::run_native(
                "Image preview",
                options,
                Box::new(|_cc| Box::new(ImagePreview::new(temp_file))),
            ) {
                eprintln!("An error occurred: {}", e);
            }

        }
    }
    Ok(())
}

/// Image preview struct for eframe
struct ImagePreview {
    image: RetainedImage,
}

impl ImagePreview {
    fn new(temp_file: PathBuf) -> Self {
        let image_data = fs::read(&temp_file).expect("Failed to read temp image file");
        fs::remove_file(temp_file).expect("Failed to delete temp file");
        Self {
            image: RetainedImage::from_image_bytes("temp_image", &image_data).unwrap(),
        }
    }
}

impl eframe::App for ImagePreview {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.image.show(ui);
        });
    }
}
