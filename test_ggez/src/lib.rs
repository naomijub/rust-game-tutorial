use ggez::graphics::set_fullscreen;
use image::DynamicImage as Image;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

pub mod screenshot;

use ggez::{event, Context};
use screenshot::get_screenshot;

#[derive(Debug, Clone)]
pub enum ScreenshotError {
    LoadIoError,
    SaveIoError,
    EncodingError,
    DecodingError,
    CaptureError,
    NoReferenceScreenshot((u32, u32, Vec<u8>)),
    ScreenshotMismatch(u32, u32, Image, Image),
}

fn write_image(filename: &str, buff: &[u8], w: u32, h: u32) -> Result<(), ScreenshotError> {
    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(filename)
        .unwrap();
    let ref mut wt = BufWriter::new(file);
    let mut encoder = png::Encoder::new(wt, w, h);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buff).unwrap();
    Ok(())
}

fn load_reference(path: &str) -> Result<Image, ScreenshotError> {
    if Path::new(path).exists() {
        println!("Loading image: {}", path);

        image::open(path).or(Err(ScreenshotError::LoadIoError))
    } else {
        println!("Failed to load test");
        Err(ScreenshotError::LoadIoError)
    }
}

fn compare_screenshot_images(
    w: u32,
    h: u32,
    reference_image: Image,
    actual_image: Image,
) -> Result<(), ScreenshotError> {
    println!("Comparing images");
    if reference_image.dimensions() == actual_image.dimensions()
        && reference_image.to_rgba8() == actual_image.to_rgba8()
    {
        println!("eq images");
        Ok(())
    } else {
        println!("diff images");
        Err(ScreenshotError::ScreenshotMismatch(
            w,
            h,
            actual_image,
            reference_image,
        ))
    }
}

fn diff_images(actual: &Image, expected: &Image) -> Result<Vec<u8>, ScreenshotError> {
    let pixels: Vec<(u32, u32, Rgba<u8>)> = actual
        .pixels()
        .zip(expected.pixels())
        .map(|(actual_pixel, expected_pixel)| {
            if actual_pixel == expected_pixel {
                (actual_pixel.0, actual_pixel.1, Rgba([u8::MIN; 4]))
            } else {
                actual_pixel
            }
        })
        .collect();

    Ok(pixels.into_iter().map(|b| b.2 .0).flatten().collect())
}

fn handle_screenshot_error(
    output_path: &str,
    screenshot_error: ScreenshotError,
) -> Result<(), ScreenshotError> {
    fs::create_dir_all(output_path).unwrap();

    match screenshot_error.clone() {
        ScreenshotError::NoReferenceScreenshot((w, h, buff)) => {
            println!("load ref err");
            write_image(&(output_path.to_string() + "expected.png"), &buff, w, h)?;
        }
        ScreenshotError::ScreenshotMismatch(w, h, ref actual, ref expected) => {
            println!("diff images err");
            let diff_image = diff_images(&actual, &expected)?;
            let actual = actual
                .pixels()
                .map(|b| b.2 .0)
                .flatten()
                .collect::<Vec<u8>>();
            let expected = expected
                .pixels()
                .map(|b| b.2 .0)
                .flatten()
                .collect::<Vec<u8>>();
            write_image(&(output_path.to_string() + "actual.png"), &actual, w, h)?;
            write_image(&(output_path.to_string() + "expected.png"), &expected, w, h)?;
            write_image(&(output_path.to_string() + "diff.png"), &diff_image, w, h)?;
        }
        _ => {}
    }
    Err(screenshot_error)
}

pub fn screenshot_test(path: &str) -> Result<(), ScreenshotError> {
    let current_dir = std::env::current_dir().unwrap();
    let resouces_path = current_dir
        .to_str()
        .map(|s| String::from(s) + "/test_resources/" + path + "/")
        .unwrap_or_default();
    let expected_path = resouces_path.clone() + "expected.png";

    let (w, h, captured_buff) = get_screenshot();
    let captured_image_buff =
        ImageBuffer::from_vec(w as u32, h as u32, captured_buff.clone()).unwrap();
    let capture_image = Image::ImageRgba8(captured_image_buff);

    fs::create_dir_all(&resouces_path).unwrap();

    let test = match load_reference(&expected_path) {
        Ok(reference_image) => Ok((reference_image, capture_image)),
        Err(_) => Err(ScreenshotError::NoReferenceScreenshot((
            w as u32,
            h as u32,
            captured_buff,
        ))),
    }
    .and_then(|(reference_image, captured_image)| {
        compare_screenshot_images(w as u32, h as u32, reference_image, captured_image)
    })
    .or_else(|err| handle_screenshot_error(&resouces_path, err));

    assert!(test.is_ok());
    Ok(())
}

pub struct TestState<T: ggez::event::EventHandler> {
    element: T,
    frame_count: usize,
    test_name: String,
}

impl<T: ggez::event::EventHandler> TestState<T> {
    pub fn new(element: T, test_name: &str) -> Self {
        Self {
            frame_count: 0,
            element,
            test_name: test_name.to_owned(),
        }
    }
}

impl<T: ggez::event::EventHandler> ggez::event::EventHandler for TestState<T> {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        if self.frame_count == 0 {
            set_fullscreen(ctx, ggez::conf::FullscreenType::Desktop)?;
            self.frame_count += 1;
        } else if self.frame_count == 2 {
            screenshot_test(&self.test_name).unwrap();
            self.frame_count += 1;
        } else if self.frame_count > 4 {
            event::quit(ctx);
        } else {
            self.frame_count += 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.83, 0.69, 0.51, 1.0].into());
        self.element.draw(ctx)?;
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}
