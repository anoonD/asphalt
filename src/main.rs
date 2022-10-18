use serde::Deserialize;
use image::GenericImageView;

#[derive(Deserialize, Copy)]
struct ColorRgba {
    name: String,
    r: f64,
    g: f64,
    b: f64,
}

fn pixel_difference(pallete: Vec<ColorRgba>, c1: ColorRgba) -> ColorRgba {
    let mut lowest_col = pallete[0];
    let mut lowest_diff = single_difference(c1, lowest_col);
    
    for c2 in 1..pallete.len() {
        let temp_diff = single_difference(c1, pallete[c2]);
        if  temp_diff < lowest_diff {
            lowest_col = pallete[c2];
        }
    }

    lowest_col
}

fn single_difference(c1: ColorRgba, c2: ColorRgba) -> f64 {
    let avg_r = 0.5 * (c1.r - c2.r); // Red average    
    let diff = f64::sqrt(
        ( 2.0 + avg_r/256.0 ) * f64::powi(c1.r - c2.r, 2)
        + 4.0 * f64::powi(c1.g - c2.g, 2)
        + ( 2.0 + (255.0 - avg_r)/256.0) * f64::powi(c1.b - c2.b, 2)
    );
    diff
}

fn main() {
    let mut csv = csv::Reader::from_path("res/colors.csv").unwrap(); // Opening color csv file
    let mut pallete: Vec<ColorRgba> = Vec::new(); // Creaing new vector to copy data from csv file

    for i in csv.deserialize() {
        // Serializing the data into the vector
        pallete.push(i.unwrap());
    }

    let mut img = image::open("res/image.jpg").unwrap(); // Opening new image for reading

    // Crop and shrink image
    if img.width() > img.height() {
        img = img
            .crop(
                (img.width() / 2) - (img.height() / 2),
                0,
                img.height(),
                img.height(),
            )
            .resize_exact(32, 32, image::imageops::FilterType::Triangle);
    } else if img.height() > img.width() {
        img = img
            .crop(
                0,
                (img.height() / 2) - (img.width() / 2),
                img.width(),
                img.width(),
            )
            .resize_exact(32, 32, image::imageops::FilterType::Triangle);
    } else {
        img = img.resize_exact(32, 32, image::imageops::FilterType::Triangle);
    }

    img.save("res/output.png").unwrap(); // Save image

}