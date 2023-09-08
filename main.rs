#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;

extern crate css_color_parser;

use colors_transform::Rgb;
use image;
use image::GenericImageView;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use skia_safe::{
    AlphaType, Color4f, ColorType, EncodedImageFormat, ImageInfo, Paint, Rect, Surface,
};

use css_color_parser::Color as CssColor;

static TEMP_RESULT_PATH: &str = "temp.png";

fn vec_to_u32_ne(bytes: &[u8]) -> u32 {
    let mut result = [0u8; 4];
    result.copy_from_slice(bytes);
    u32::from_ne_bytes(result)
}

fn png_to_bruh(path: PathBuf) -> Result<(), std::io::Error> {
    let img = image::open(&path).expect("File not found!");
    let mut str = String::new();
    let mut last_line = 0;

    for pixel in img.pixels() {
        let hex_color = Rgb::from(
            pixel.2 .0[0] as f32,
            pixel.2 .0[1] as f32,
            pixel.2 .0[2] as f32,
        )
        .to_css_hex_string();

        if last_line != pixel.1 {
            str.push_str("\n");
            last_line = pixel.1;
        }
        str.push_str(&hex_color.replace("#", ""));
    }

    if let Some(path_str) = &path.to_str() {
        let height: u32 = img.height();
        let width: u32 = img.width();

        let height_bytes: [u8; 4] = height.to_ne_bytes();
        let width_bytes: [u8; 4] = width.to_ne_bytes();
        let path_to_bruh = path_str.replace(".png", ".bruh");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path_to_bruh)
            .expect("Couldnt write");
        let string_bytes: Vec<u8> = Vec::from(str.as_bytes());

        file.write_all(&width_bytes).unwrap();
        file.write_all(&height_bytes).unwrap();
        file.write_all(&string_bytes).unwrap();
        file.flush().unwrap();
    } else {
        println!("{}", "couldn't find")
    }

    Ok(())
}

fn bruh_to_png(path: PathBuf) -> (u32, u32) {
    let mut contents: Vec<u8> = fs::read(&path).expect("Couldn't read file.");
    let binding: Vec<_> = contents.drain(0..8).collect();

    let width = vec_to_u32_ne(&binding[0..4]);
    let height = vec_to_u32_ne(&binding[4..8]);

    let sanitized_content = String::from_utf8_lossy(&contents).replace("\n", "");

    let result: Vec<&str> = sanitized_content
        .as_bytes()
        .chunks(6)
        .map(std::str::from_utf8)
        .collect::<Result<_, _>>()
        .expect("Invalid UTF-8 sequence in the input string");

    let info = ImageInfo::new(
        (width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Opaque,
        None,
    );

    let mut surface = Surface::new_raster(&info, None, None).unwrap();
    let canvas = surface.canvas();

    for (i, color) in result.iter().enumerate() {
        let hex = "#".to_owned() + color;

        let parsed_color = hex
            .parse::<CssColor>()
            .expect("Failed to convert Hex to RGB");
        let color4f = Color4f::new(
            parsed_color.r as f32,
            parsed_color.g as f32,
            parsed_color.b as f32,
            0.004 as f32,
        );
        let paint = Paint::new(color4f, None);
        if i == 0 {
            println!("{:?}", paint)
        }
        let x = i % width as usize;
        let y = i / width as usize;

        let rect = Rect::from_point_and_size((x as f32, y as f32), (1.0, 1.0));
        canvas.draw_rect(rect, &paint);
    }

    let image = surface.image_snapshot();

    if let Some(data) = image.encode(None, EncodedImageFormat::PNG, 100) {
        fs::write(TEMP_RESULT_PATH, &*data).expect("Failed to write image data to file");
    }

    return (width, height);
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path: PathBuf = (&args[1]).into();

    if &args[1] == "compile" {
        if args.len() < 3 {
            panic!("Secondary argument ('path') not provided. Example: `cargo run compile ~/image.png`")
        }

        let path: PathBuf = (&args[2]).into();

        match png_to_bruh(path) {
            Ok(()) => println!("{}", "Successfully converted PNG to BRUH"),
            Err(_) => println!("{}", "Failed to convert PNG to BRUH"),
        }

        Ok(())
    } else {
        let (width, height) = bruh_to_png(file_path);
        println!("{} {}", width, height);
        let options = eframe::NativeOptions {
            resizable: false,
            initial_window_size: Some(egui::vec2(width as f32, height as f32)),
            ..Default::default()
        };

        eframe::run_native(
            "Image preview",
            options,
            Box::new(|_cc| Box::<ImagePreview>::default()),
        )
    }
}
struct ImagePreview {
    image: RetainedImage,
}

impl Default for ImagePreview {
    fn default() -> Self {
        let image_data = std::fs::read(TEMP_RESULT_PATH).expect("Failed to read image file");

        fs::remove_file(TEMP_RESULT_PATH).expect("File delete failed on TEMP_RESULT_PATH");

        Self {
            image: RetainedImage::from_image_bytes(TEMP_RESULT_PATH, &image_data).unwrap(),
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
