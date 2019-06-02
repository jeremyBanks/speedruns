use std::convert::TryFrom;

use env_logger;
use image::{self, DynamicImage, ImageBuffer, Rgb};
#[allow(unused)] use log::{debug, error, info, trace, warn};
use rug::{Assign, Complex, Float, Rational};
use serde::Serialize;

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

#[derive(Debug, Serialize)]
pub struct View {
    pub real:       Rational,
    pub imag:       Rational,
    pub diameter:   Rational,
    pub resolution: u32,
}

#[derive(Debug, Serialize)]
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
            real: Rational::from((-21, 32)),
            imag: Rational::from((-16, 32)),
            // width of rendered area
            diameter: Rational::from((1, 1 << 14)),
            // width of rendered image
            resolution: 512,
        }
    }
}

impl View {
    pub fn precision(&self) -> u32 {
        128
    }

    pub fn iteration_limit(&self) -> u32 {
        64
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = ImageBuffer::new(self.resolution, self.resolution);

        let half_resolution = Rational::from(self.resolution.clone()) / Rational::from(2);
        let half_diameter = Rational::from(&self.diameter) / Rational::from(2);
        let pixel_offset = &self.diameter / Rational::from(self.resolution - 1);
        let real_left = Rational::from(&self.real - &half_diameter);
        let imag_top = Rational::from(&self.imag - &half_diameter);

        for x in 0..self.resolution {
            let real = &real_left + Rational::from(x) * &pixel_offset;
            for y in 0..self.resolution {
                let imag = &imag_top + Rational::from(y) * &pixel_offset;

                let point = self.point(real.clone(), imag);
                let color = point.color();

                image.put_pixel(x, y, color);
            }
        }

        DynamicImage::ImageRgb8(image)
    }

    pub fn point(&self, real: Rational, imag: Rational) -> Point {
        let escape_magnitude = Float::with_val(self.precision(), 2);

        let mut c = Complex::from((
            Float::with_val(self.precision(), &real),
            Float::with_val(self.precision(), &imag),
        ));

        let mut z_n = c.clone();
        let mut z_n_minus_one = z_n.clone();
        let mut escape = None;

        for i in 0..self.iteration_limit() {
            let magnitude = z_n.magnitude();
            if magnitude > escape_magnitude {
                escape = Some((i, magnitude));
                break
            }

            z_n_minus_one.assign(&z_n);
            z_n.assign((&z_n_minus_one * &z_n_minus_one) + &c);
        }

        Point { real, imag, escape }
    }
}

impl Point {
    pub fn color(&self) -> Rgb<u8> {
        Rgb(match self.escape {
            Some((iterations, ref magnitude)) => [
                0,
                u8::try_from(iterations).unwrap_or(255),
                u8::try_from(iterations * 16).unwrap_or(255),
            ],
            None => [255, 255, 255],
        })
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

    let r = Rational::from((1, 2));
    let f = Float::with_val(8, r);

    Ok(())
}
