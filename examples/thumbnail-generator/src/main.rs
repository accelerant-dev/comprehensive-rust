use image::imageops::FilterType;

fn main() {
    let mut args = std::env::args();

    let filename = args.nth(1).expect("filename required");

    let img = image::open(filename).unwrap();

    let thumbnail = img.resize(40, 30, FilterType::Nearest);

    thumbnail.save("output.jpg").unwrap();
}
