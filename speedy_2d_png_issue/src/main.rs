
use image::*;
use image::io::Reader as ImageReader;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main()
{
    let window = Window::new_centered("Speedy2D: Load image", (800, 600)).unwrap();

    window.run_loop(MyWindowHandler { image: None })
}

struct MyWindowHandler
{
    image: Option<ImageHandle>
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::WHITE);
        let img: DynamicImage = ImageReader::open("imgs/smiley.png").unwrap().decode().unwrap();
        let (ci_width, ci_height) = img.dimensions();
        let has_alpha = has_alpha(&img);
        
        let ci_bytes = match has_alpha {
            true => img.into_rgba8().into_raw(),
            false => img.into_rgb8().into_raw()
        };

        let img_type = match has_alpha {
            true => speedy2d::image::ImageDataType::RGBA,
            false => speedy2d::image::ImageDataType::RGB,
        };
        let to_draw_image = match graphics.create_image_from_raw_pixels(
                img_type,
                ImageSmoothingMode::NearestNeighbor,
                Vector2::new(ci_width, ci_height),
                &ci_bytes,
            ){
                Ok(img) => img,
                Err(e) => panic!("Error creating image {}", e)
        };
        graphics.draw_image((200.0, 0.0), &to_draw_image);

    }
}

fn has_alpha(img: &DynamicImage) -> bool{
    img.color().has_alpha()
}
