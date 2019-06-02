use std::convert::TryFrom;

use env_logger;
use image::{self, DynamicImage, ImageBuffer, Rgb};
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use rug::{Assign, Complex, Float, Rational};
use serde::Serialize;

macro_rules! rat {
    ( $n:tt / $d:tt ) => {
        Rational::from(($n, $d))
    };
    ( $n:expr ) => {
        Rational::from($n)
    };
}

trait ComplexUtils {
    fn magnitude(&self) -> Float;
}

impl ComplexUtils for Complex {
    fn magnitude(&self) -> Float {
        Float::with_val(
            self.real().prec(),
            (self.real() * self.real()) + (self.imag() * self.imag()),
        )
        .sqrt()
    }
}

#[derive(Debug)]
pub struct View {
    pub real:       Rational,
    pub imag:       Rational,
    pub diameter:   Rational,
    pub resolution: u32,
}

#[derive(Debug)]
pub struct Point {
    pub real: Rational,
    pub imag: Rational,
    // the number of iterations and then distance of the escape,
    // or none if the point is in the mandelbrot set.
    pub escape: Option<(u32, Float)>,
}

impl Default for View {
    fn default() -> Self {
        View {
            // center of rendered area
            real: rat!(400 / 1024),
            imag: rat!(270 / 1024),
            // width of rendered area
            diameter: rat!(1 / (1 << 16)),
            // width of rendered image
            resolution: 512,
        }
    }
}

impl View {
    pub fn precision(&self) -> u32 {
        32 + ((rat!(1) / &self.diameter).to_f64().log2() as u32)
    }

    pub fn iteration_limit(&self) -> u32 {
        256
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = ImageBuffer::new(self.resolution, self.resolution);

        let half_diameter = rat!(&self.diameter) / rat!(2);
        let pixel_offset = &self.diameter / rat!(self.resolution - 1);
        let real_left = rat!(&self.real - &half_diameter);
        let imag_top = rat!(&self.imag - &half_diameter);

        let mut points = vec![];

        for x in 0..self.resolution {
            let real = &real_left + rat!(x) * &pixel_offset;
            for y in 0..self.resolution {
                let imag = &imag_top + rat!(y) * &pixel_offset;

                let point = self.point(real.clone(), imag);
                points.push((x, y, point));
            }
        }

        let color_map =
            ColorMap::new(points.iter().map(|(_x, _y, point)| point.magnitude()));

        for (x, y, point) in points.iter() {
            let magnitude = point.magnitude();
            let color = color_map.color(magnitude);

            image.put_pixel(*x, *y, color);
        }

        DynamicImage::ImageRgb8(image)
    }

    pub fn point(&self, real: Rational, imag: Rational) -> Point {
        let escape_magnitude = Float::with_val(self.precision(), 2);

        let c = Complex::from((
            Float::with_val(self.precision(), &real),
            Float::with_val(self.precision(), &imag),
        ));

        let mut z_n = c.clone();
        let mut z_n_minus_one = z_n.clone();
        let mut escape = None;

        for i in 0..self.iteration_limit() {
            let magnitude = z_n.magnitude();
            if magnitude >= escape_magnitude {
                escape = Some((i, magnitude - escape_magnitude));
                break
            }

            z_n_minus_one.assign(&z_n);
            z_n.assign((&z_n_minus_one * &z_n_minus_one) + &c);
        }

        Point { real, imag, escape }
    }
}

impl Point {
    pub fn magnitude(&self) -> Float {
        match self.escape {
            Some((iterations, ref magnitude)) =>
                Float::with_val(32, iterations) - Float::with_val(32, magnitude) / 2.1,
            None =>
                Float::with_val(32, -4)
                    + Float::with_val(32, &self.real).abs()
                    + Float::with_val(32, &self.imag).abs(),
        }
    }
}

#[derive(Debug)]
pub struct ColorMap {
    pub magnitude_min: rug::Float,
    pub magnitude_max: rug::Float,
}

impl ColorMap {
    pub fn new<'a>(values: impl Iterator<Item = Float>) -> Self {
        let values = values
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect::<Vec<_>>();

        let twentith = values.len() / 20;

        Self {
            magnitude_min: values[twentith].clone(),
            magnitude_max: values[values.len() - 1 - twentith].clone(),
        }
    }

    pub fn color(&self, value: Float) -> image::Rgb<u8> {
        if value.is_sign_negative() {
            return Rgb([255, 255, 255])
        }

        let range =
            Float::with_val(value.prec(), &self.magnitude_max - &self.magnitude_min);
        let range_log = range.clone().log2();

        let value_normalized =
            Float::with_val(value.prec(), &value - &self.magnitude_min) / range;
        let value_log_normalized =
            Float::with_val(value.prec(), &value - &self.magnitude_min).log2() / range_log;

        let value_u16 = (value_normalized * Float::with_val(32, 0xFF_FF))
            .to_u32_saturating()
            .unwrap();
        let value_log_u16 = (value_log_normalized * Float::with_val(32, 0xFF_FF))
            .to_u32_saturating()
            .unwrap_or(0);

        Rgb([
            u8::try_from(value_u16 >> 8).unwrap_or(255),
            u8::try_from(value_log_u16 >> 8).unwrap_or(255),
            u8::try_from(value_u16 >> 4).unwrap_or(255),
        ])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace,mandelbrot=trace", module_path!())),
    )?;

    let x = View::default();
    let i = x.render();
    i.save("./target/mandelbrot.png")?;

    let r = rat!((1, 2));
    let f = Float::with_val(8, r);

    Ok(())
}
