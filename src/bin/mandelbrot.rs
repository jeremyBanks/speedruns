use std::convert::TryFrom;

use env_logger;
use image::{self, DynamicImage, ImageBuffer, Rgb};
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use palette::{encoding::pixel::Pixel, LabHue, Lch, Srgb};
use rug::{Assign, Complex, Float, Rational};

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

#[derive(Debug, Clone)]
pub struct View {
    pub real:       Rational,
    pub imag:       Rational,
    pub diameter:   Rational,
    pub resolution: u32,
}

impl View {
    pub fn default() -> Self {
        View {
            real:       rat!(-1) / rat!(2),
            imag:       rat!(0),
            diameter:   rat!(3),
            resolution: 1024 * 4,
        }
    }

    pub fn deep() -> Self {
        View {
            // center of rendered area
            real: rat!(400 / 1024) - rat!(1 / (1 << 21)) - rat!(1 / (1 << 25))
                + rat!(1 / (1 << 29))
                - rat!(1 / (1u64 << 41))
                - rat!(1 / (1u64 << 49))
                + rat!(1 / (1u64 << 55))
                - rat!(1 / (1u64 << 63)),
            imag: rat!(270 / 1024) - rat!(1 / (1 << 25)) - rat!(1 / (1u64 << 63)),
            // width of rendered area
            diameter: rat!(1 / (1u64 << 63)),
            // width of rendered image
            resolution: 256,
        }
    }

    pub fn pixel_size(&self) -> Rational {
        &self.diameter / Rational::from(self.resolution)
    }

    /// the number of bits of precision required for the floating point calculations
    /// to simulate this view. (this only applies the significand/value digits, the
    /// exponent is a bigint in all cases.) this was sort-of a guess.
    pub fn required_precision(&self) -> u32 {
        64 + ((rat!(1) / self.pixel_size()).to_f64().log2() as u32)
    }
}

#[derive(Debug, Clone)]
pub struct State {
    /// the view this state is simulating
    pub view: View,
    /// the number of iterations of the mandelbrot loop that we've done so far
    pub iterations: u32,
    /// a [y][x] 2D array of the origin positions for each cell/pixel
    pub origins: Vec<Vec<Complex>>,
    /// a [y][x] 2D array of the latest position for each cell/pixel
    pub positions: Vec<Vec<Complex>>,
}

impl State {
    pub fn new(view: View) -> State {
        let half_diameter = rat!(&view.diameter) / rat!(2);
        let pixel_size = view.pixel_size();
        let prec = view.required_precision();
        let real_left = rat!(&view.real - &half_diameter);
        let imag_top = rat!(&view.imag - &half_diameter);

        let origins: Vec<Vec<Complex>> = (0..view.resolution)
            .map(|y| {
                let imag = &imag_top + rat!(y) * &pixel_size;
                let imag = Float::with_val(prec, imag);
                (0..view.resolution)
                    .map(|x| {
                        let real = &real_left + rat!(x) * &pixel_size;
                        let real = Float::with_val(prec, real);
                        let point = Complex::from((real, imag.clone()));

                        point
                    })
                    .collect()
            })
            .collect();

        State {
            view: view.clone(),
            iterations: 0,
            positions: origins.clone(),
            origins,
        }
    }

    pub fn iterate(&mut self) {
        for x in 0..self.view.resolution {
            let x = x as usize;
            for y in 0..self.view.resolution {
                let y = y as usize;
                let origin = &self.origins[y][x];
                let previous = self.positions[y][x].clone();
                self.positions[y][x].assign(previous.square() + origin);
            }
        }
        self.iterations += 1;
    }

    // pub fn iteration_limit(&self) -> u32 {
    //     6
    // }

    pub fn render(&mut self) -> DynamicImage {
        let mut image = ImageBuffer::new(self.view.resolution, self.view.resolution);

        // while self.iterations <= self.iteration_limit() {
        //     self.iterate();
        // }

        for x in 0..self.view.resolution {
            for y in 0..self.view.resolution {
                let position = &self.positions[y as usize][x as usize];
                let mut radians = position.imag().clone().atan2(position.real());
                if !radians.is_finite() {
                    radians = Float::new(32);
                }

                let mut mag = position.magnitude().to_f64();
                if !mag.is_finite() || mag > 2.0 {
                    mag = 2.0;
                }
                let color = Srgb::new(0.125, 0.25, 0.5);
                let mut color = Lch::from(color);
                // color.chroma = 50.0 + 256.0 / mag.log2();
                // color.chroma = mag.log2() * 200.0;  //* 128.0;
                // color.l = mag.log2() * 100.0; // * (100.0 / 10.0);
                color.l = (mag + 1.0).log2() * 75.0;
                color.hue = LabHue::from_radians(radians.to_f64());

                let color: [u8; 3] = Srgb::from(color).into_format().into_raw();
                image.put_pixel(x, y, Rgb(color));
            }
        }

        DynamicImage::ImageRgb8(image)
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
            .filter(|x| !x.is_sign_negative())
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect::<Vec<_>>();

        assert!(values.len() > 0);

        let twentith = values.len() / 8;

        let min = values[twentith].clone();
        let max = values[values.len() - 1 - twentith].clone();
        let range = max.clone() - &min;
        let min = min - (range.clone() / 4);
        let max = max + (range.clone() / 4);

        Self {
            magnitude_min: min,
            magnitude_max: max,
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

        let value_u16 = (value_normalized * Float::with_val(64, 0xFF_FF))
            .to_u32_saturating()
            .unwrap();
        let value_log_u16 = (value_log_normalized * Float::with_val(64, 0xFF_FF))
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

    let view = View::default();
    let mut state = State::new(view);
    for n in 0..128 {
        trace!("iteration {}", n);
        let image = state.render();
        image.save(format!("./target/mandelbrot-{:03}.png", n))?;
        state.iterate();
    }

    Ok(())
}
