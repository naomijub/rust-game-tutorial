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

fn write_image( filename: &str, img: &Image) -> Result<(), ScreenshotError> {
    img.save_with_format(filename, image::ImageFormat::Png)
        .or(Err(ScreenshotError::SaveIoError))?;
    Ok(())
}

fn load_reference( path: &str) -> Result<Image, ScreenshotError> {
    if Path::new(path).exists() {
        println!("Loading image: {}", path);

        image::open(path).or(Err(ScreenshotError::LoadIoError))
    } else {
        println!("Failed to load test");
        Err(ScreenshotError::LoadIoError)
    }
}

fn compare_screenshot_images(
    
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
    
    actual: &Image,
    expected: &Image,
) -> Result<Image, ScreenshotError> {
    let dimensions = actual.dimensions();
    let width = dimensions.0;
    let height = dimensions.1;

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
    let mut im = ImageBuffer::new(width, height);
    for (x, y, pixel) in pixels {
        im.put_pixel(x, y, pixel);
    }

    Ok(Image::ImageRgba8(im))
}

fn handle_screenshot_error(
    output_path: &str,
    screenshot_error: ScreenshotError,
) -> Result<(), ScreenshotError> {
    fs::create_dir_all(output_path).unwrap();
   
    match screenshot_error {
        ScreenshotError::NoReferenceScreenshot(ref img) => {
            println!("load ref err");
            write_image( &(output_path.to_string() + "expected.png"), &img)?;
        }
        ScreenshotError::ScreenshotMismatch(ref actual, ref expected) => {
            println!("diff images err");
            let diff_image = diff_images( &actual, &expected)?;
            write_image( &(output_path.to_string() + "actual.png"), &actual)?;
            write_image( &(output_path.to_string() + "expected.png"), &expected)?;
            write_image( &(output_path.to_string() + "diff.png"), &diff_image)?;
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
    let actual_path = resouces_path.clone() + "actual.png";

    let (w, h, captured_buff) = get_screenshot();
    let captured_image_buff = ImageBuffer::from_vec(w as u32,h as u32, captured_buff).unwrap();
    let capture_image = Image::ImageBgra8(captured_image_buff);
    capture_image.save_with_format(actual_path, image::ImageFormat::Png).unwrap();
    
    println!("{:?}", capture_image);
    let test = match load_reference(&expected_path) {
        Ok(reference_image) => Ok((reference_image, capture_image)),
        Err(_) => Err(ScreenshotError::NoReferenceScreenshot(capture_image)),
    }
    .and_then(|(reference_image, captured_image)| {
        compare_screenshot_images( reference_image, captured_image)
    })
    .or_else(|err| handle_screenshot_error( &resouces_path, err));

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
