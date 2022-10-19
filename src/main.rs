use std::f32::consts::E;
use std::fs::File;
use std::io::{Error, Write};

use image::{GenericImage, GenericImageView};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
struct ColorRgba {
    name: String,
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRgba {
    pub fn to_rgba(color: image::Rgba<u8>) -> ColorRgba {
        ColorRgba {
            name: String::new(),
            r: color.0[0] as f64,
            g: color.0[1] as f64,
            b: color.0[2] as f64,
        }
    }
}

fn color_difference(pallete: &Vec<ColorRgba>, c1: ColorRgba) -> ColorRgba {
    let mut lowest: Option<ColorRgba> = None;
    let mut lowest_diff: f64 = f64::MAX;

    for c2 in pallete.iter() {
        // Start Formula

        let avg_r = 0.5 * (c1.r + c2.r);
        let delta_r = f64::abs(c1.r*1.1 - c2.r);
        let delta_g = f64::abs(c1.g - c2.g);
        let delta_b = f64::abs(c1.b - c2.b);

        let temp_diff =
            f64::sqrt(
                (2.0 + avg_r/256.0) * delta_r.powi(2)
                + 4.0 * delta_g.powi(2)
                + (2.0 + (255.0-avg_r)/256.0) * delta_b.powi(2)
            );
        
        // End Formula
        
        if temp_diff < lowest_diff {
            lowest_diff = temp_diff;
            lowest = Some(c2.clone());
        }
    }

    lowest.unwrap()
}

fn main() {
    let mut img = image::open("res/image07.jpg").unwrap(); // Opening new image for reading

    let mut csv = csv::Reader::from_path("res/colors.csv").unwrap(); // Opening color csv file
    let mut pallete: Vec<ColorRgba> = Vec::new(); // Creaing new vector to copy data from csv file

    for i in csv.deserialize() {
        // Serializing the data into the vector
        pallete.push(i.unwrap());
    }

    // Crop and shrink image
    if img.width() > img.height() {
        img = img
            .crop(
                (img.width() / 2) - (img.height() / 2),
                0,
                img.height(),
                img.height(),
            )
            .thumbnail_exact(32, 32);
    } else if img.height() > img.width() {
        img = img
            .crop(
                0,
                (img.height() / 2) - (img.width() / 2),
                img.width(),
                img.width(),
            )
            .thumbnail_exact(32, 32);
    } else {
        img = img.thumbnail_exact(32, 32);
    }

    // img.save("res/output.png").unwrap(); // Save image
    // println!("{:?}", img.get_pixel(0, 0));

    let mut export = String::from("String[][] temp = ");

    export.push_str("{");
    for x in 0..32 {
        export.push_str("{");
        for y in 0..32 {
            export.push('"');
            export.push_str(color_difference(&pallete, ColorRgba::to_rgba(img.get_pixel(x, y))).name.as_str());
            export.push('"');
            export.push(',');

        }
        export.push_str("}, ");
    }
    export.push_str("};");

    // img.save("res/output_2.png").unwrap(); // Save image
    // println!("{:?}", img.get_pixel(0, 0));

    let mut path = File::create("output.txt").unwrap();
    write!(path, "{}", export);

}
