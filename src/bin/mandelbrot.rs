#![allow(unused_imports)]
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};

use image::{Pixel, Rgb, self};
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use rug::{Rational, Float, Complex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace,mandelbrot=trace", module_path!())),
    )?;

    let real = Rational::from((-7463, 10000));
    let imag = Rational::from((1102, 10000));
    let radius = Rational::from((5, 1000));
    let radius_pixels = 127;
    let mandelbrot_view = View { real, imag, radius, radius_pixels };
    trace!("{:#?}", mandelbrot_view);

    let image = mandelbrot_view.render();
    image.save("./target/mandelbrot.png")?;
    trace!("saved target/mandelbrot.png");

    // let width = 100;
    // let height = 12;
    // for y in 0..height {
    //     let y = rug::Rational::from((y - height / 2, height / 2));
    //     for x in 0..width {
    //         let x = rug::Rational::from((x - width / 2, width / 2));
    //         // TODO: variable precision instead of 256
    //         let mut c = rug::Complex::new(256);
    //         *c.mut_real() += &x;
    //         *c.mut_imag() += &y;
    //         let m = mandelbrot(c);
    //         if m.is_in_the_mandelbrot_set {
    //             print!("X");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }

    Ok(())
}


#[derive(Debug, Clone)]
pub struct View {
    /// Real component of center coordinate, rendered as x.
    pub real:      Rational,
    /// Imaginary component of center coordinate, rendered as y.
    pub imag:      Rational,
    /// The radius of the (smallest circle spanning the) rendered region.
    pub radius:    Rational,
    /// And the number of pixels we use when rendering that distance.
    pub radius_pixels: u32,
}

impl View {
    pub fn new(real: Rational, imag: Rational, radius: Rational, radius_pixels: u32) -> Self {
        Self {
            real,
            imag,
            radius,
            radius_pixels
        }
    }

    // pub fn iter(&self) -> impl Iterator<Item=()

    /// The precision (in bits) that will be used for calculating this render,
    /// to make there are no noticable float atifacts.
    pub fn precision(&self) -> u32 {
        128
    }

    pub fn render(&self) -> image::DynamicImage {
        let size = self.radius_pixels * 2 + 1;
        let buffer = image::DynamicImage::ImageRgb8(image::ImageBuffer::new(size, size));

        buffer
    }
}

#[derive(Debug, Clone)]
struct Point {
    pub coordinate:               Complex,
    pub escape_time: u32,
    pub escape_distance: Float,
    pub is_in_the_mandelbrot_set: bool,
}

impl Point {
    pub fn calculate(coordinate: Complex) -> Self {
        let mut is_in_the_mandelbrot_set = false;

        let mut z = coordinate.clone();

        for _iteration in 0..10 {
            let last_z = z.clone();
            z = z.mul_add(&last_z, &coordinate);

            if z.real() > &2 || z.imag() > &2 {
                is_in_the_mandelbrot_set = true;
                break
            }
        }

        Self {
            coordinate,
            is_in_the_mandelbrot_set,
        }
    }

    pub fn color(&self) -> Rgb<u8> {
        let rgb;
        if !self.is_in_the_mandelbrot_set {
            rgb = [0, 0, 0];
        } else {
            rgb = [255, 255, 255];
        }
        Rgb { data: rgb }
    }
}