use ggez::graphics::screen_coordinates;
use ggez::graphics::set_fullscreen;
use image::DynamicImage as Image;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use std::fs;
use std::path::Path;

pub mod screenshot;

use ggez::{event, Context};
use screenshot::get_screenshot;

#[derive(Debug)]
pub enum ScreenshotError {
    LoadIoError,
    SaveIoError,
    EncodingError,
    DecodingError,
    CaptureError,
    NoReferenceScreenshot(Image),
    ScreenshotMismatch(Image, Image),
}

fn write_image(ctx: &mut Context, filename: &str, img: &Image) -> Result<(), ScreenshotError> {
    // let options = ggez::filesystem::OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true);
    // let _ = ggez::filesystem::open_options(ctx, &filename, options)
    //     .or(Err(ScreenshotError::SaveIoError))?;
    // img.encode(ctx, ImageFormat::Png, filename)
    //     .or(Err(ScreenshotError::SaveIoError))?;
    img.save_with_format(filename, image::ImageFormat::Png)
        .or(Err(ScreenshotError::SaveIoError))?;
    Ok(())
}

fn load_reference(ctx: &mut Context, path: &str) -> Result<Image, ScreenshotError> {
    if Path::new(path).exists() {
        println!("Loading image: {}", path);

        image::open(path).or(Err(ScreenshotError::LoadIoError))
    } else {
        println!("Failed to load test");
        Err(ScreenshotError::LoadIoError)
    }
}

fn compare_screenshot_images(
    ctx: &mut Context,
    reference_image: Image,
    actual_image: Image,
) -> Result<(), ScreenshotError> {
    println!("Comparing images");
    if reference_image.dimensions() == actual_image.dimensions()
        && reference_image
            .to_rgba8()
            == actual_image
                .to_rgba8()
    {
        println!("eq images");
        Ok(())
    } else {
        println!("diff images");
        Err(ScreenshotError::ScreenshotMismatch(
            actual_image,
            reference_image,
        ))
    }
}

fn diff_images(
    ctx: &mut Context,
    actual: &Image,
    expected: &Image,
) -> Result<Image, ScreenshotError> {
    let dimensions = actual.dimensions();
    let width = dimensions.0 as u16;
    let height = dimensions.1 as u16;

    let pixels: Vec<(u32, u32, Rgba<u8>)> = actual
        .pixels()
        .zip(
            expected
                .pixels()
        )
        .map(|(actual_pixel, expected_pixel)| {
            if actual_pixel == expected_pixel {
                (actual_pixel.0, actual_pixel.1 , Rgba([u8::MIN; 4]))
            } else {
                actual_pixel
            }
        })
        .collect();

    Ok(Image::ImageRgba8(ImageBuffer::new(actual.width(), actual.height()))) //from_pixel(actual.width(), actual.height(), pixels)
    // Image::from_rgba8(ctx, width, height, &pixels).or(Err(ScreenshotError::EncodingError))
}

fn handle_screenshot_error(
    ctx: &mut Context,
    output_path: &str,
    screenshot_error: ScreenshotError,
) -> Result<(), ScreenshotError> {
    fs::create_dir_all(output_path).unwrap();
    // ggez::filesystem::create_dir(ctx, output_path).or(Err(ScreenshotError::SaveIoError))?;
    // ggez::filesystem::create_dir(ctx, output_path).or(Err(ScreenshotError::SaveIoError))?;
    match screenshot_error {
        ScreenshotError::NoReferenceScreenshot(ref img) => {
            println!("load ref err");
            write_image(ctx, &(output_path.to_string() + "expected.png"), &img)?;
        }
        ScreenshotError::ScreenshotMismatch(ref actual, ref expected) => {
            println!("diff images err");
            let diff_image = diff_images(ctx, &actual, &expected)?;
            write_image(ctx, &(output_path.to_string() + "actual.png"), &actual)?;
            write_image(ctx, &(output_path.to_string() + "expected.png"), &expected)?;
            write_image(ctx, &(output_path.to_string() + "diff.png"), &diff_image)?;
        }
        _ => {}
    }
    Err(screenshot_error)
}

pub fn screenshot_test(ctx: &mut Context, path: &str) -> Result<(), ScreenshotError> {
    let sc = screen_coordinates(ctx);
    let current_dir = std::env::current_dir().unwrap();
    let resouces_path = current_dir
        .to_str()
        .map(|s| String::from(s) + "/test_resources/" + path + "/")
        .unwrap_or_default();
    let expected_path = resouces_path.clone() + "expected.png";
    // let actual_path = resouces_path.clone() + "actual.png";

    let captured_buff = get_screenshot();
    let captured_image_buff = ImageBuffer::from_vec(sc.w as u32,sc.h as u32, captured_buff).unwrap();
    let capture_image = Image::ImageBgra8(captured_image_buff);
    // println!("Captured dimensions: {:?}", captured_image.dimensions());
    println!("Image captures");
    let test = match load_reference(ctx, &expected_path) {
        Ok(reference_image) => Ok((reference_image, capture_image)),
        Err(_) => Err(ScreenshotError::NoReferenceScreenshot(capture_image)),
    }
    .and_then(|(reference_image, captured_image)| {
        compare_screenshot_images(ctx, reference_image, captured_image)
    })
    .or_else(|err| handle_screenshot_error(ctx, &resouces_path, err));

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
        } else if self.frame_count > 2 {
            screenshot_test(ctx, &self.test_name).unwrap();
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
