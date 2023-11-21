use std::io::Cursor;
use image::ImageFormat;
use image::io::Reader as ImageReader;
use std::fmt::format;

use image::imageops::FilterType;
use rouille::{self, Request, Response, input, try_or_400};
use rouille::post_input;
use rouille::input::post::BufferedFile;
use serde::Serialize;

fn main() {
    rouille::start_server("0.0.0.0:5000", move |req| {
        let params = try_or_400!(post_input!(req, {
            image: BufferedFile,
            from: Option<String>,
            x: u32,
            y: u32,
            filter: Option<String>,
        }));

        let image_data = Cursor::new(params.image.data);
        let mut reader = ImageReader::new(image_data);

        if let Some(ext) = params.from {
            if let Some(format) = ImageFormat::from_extension(ext) {
                reader.set_format(format);
            }
        }

        if reader.format().is_none() {
            if let Some(format) = ImageFormat::from_mime_type(params.image.mime) {
                reader.set_format(format);
            }
        }

        if reader.format().is_none() {
            reader = try_or_400!(reader.with_guessed_format());
        }

        let filter_raw = params.filter.unwrap_or("nearest".to_string())
        .trim().to_ascii_lowercase();
        let filter = match filter_raw.as_str() {
            "n" | "near" | "nearest" => FilterType::Nearest,
            "t" | "tri" | "triangle" => FilterType::Triangle,
            "g" | "gauss" | "gaussian" => FilterType::Gaussian,
            "c" | "cr" | "catmullrom" => FilterType::CatmullRom,
            "l" | "l3" | "lanczos" | "lanczos3" => FilterType::Lanczos3,
            _ => return Response::html("<p>Invalid filter argument</>")
                .with_status_code(400),
        };

        let fmt = &reader.format().unwrap();
        let fmt_ext = fmt.extensions_str().first().unwrap();

        if let Some(new_fmt) = params.to {
            if fmt.extensions_str().contains(&new_fmt.as_str()) {
               fmt
            } else {
                return Response::html("<p>Invalid to  argument</>")
                .with_status_code(400);
            }
        }

        let img = try_or_400!(reader.decode());
        let img = img.resize(params.x, params.y, filter);

        if let Some(filename) = params.image.filename {
            Path::from(filename)
        } else {
            Path::from(format!("resize-{}x{}.{}", params.x, params.y, fmt_ext));
        }


        // Response::json(&greeting)
        Response::empty_204()
    });


    let filter = "nearest";

    let mut args = std::env::args();

    // input handling

    let filename = args.nth(1)
        .unwrap_or("input.jpg".to_string());

    let filter = args.next()
        .unwrap_or("nearest".to_string());

    let width = args.next()
        .unwrap_or("40".to_string());

    let height = args.next()
        .unwrap_or("40".to_string());

    // conversion to rust types

    let width = width.parse::<u32>()
        .expect("cant convert x parameter as an int");

    let height = height.parse::<u32>()
        .expect("cant convert y parameter as an int");

    let filter = match filter.as_str() {
        "nearest" => FilterType::Nearest,
        _ => panic!("unknown filter type"),
    };

    // image processing

    let img = image::open(filename).unwrap();

    let thumbnail = img.resize(width, height, filter);

    thumbnail.save("output-cr.jpg").unwrap();
}