
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
        let ci_bytes: Vec<u8> = img.to_bytes();
        let (ci_width, ci_height) = img.dimensions();
        let img_type = match has_alpha(img){
            true=>speedy2d::image::ImageDataType::RGBA,
            false=>speedy2d::image::ImageDataType::RGB,
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

        //loading image with true alpha channel gives(exit code: 0xc000041d), not caught by match statement
        
        // let img_2: DynamicImage = ImageReader::open("imgs/rust-logo.png").unwrap().decode().unwrap();
        // let ci_bytes_2: Vec<u8> = img_2.to_bytes();
        // let (ci_width_2, ci_height_2) =  img_2.dimensions();

        // let img_type_2 = match has_alpha(img_2){
        //     true=>speedy2d::image::ImageDataType::RGBA,
        //     false=>speedy2d::image::ImageDataType::RGB,
        // };
        // let to_draw_image_2 = match graphics.create_image_from_raw_pixels(
        //         img_type_2,
        //         ImageSmoothingMode::NearestNeighbor,
        //         Vector2::new(ci_width_2, ci_height_2),
        //         &ci_bytes_2,
        //     ){
        //         Ok(img) => img,
        //         Err(e) => panic!("Error creating image {}", e)
        // };
        // graphics.draw_image((0.0, 0.0), &to_draw_image_2);
    }
}

fn has_alpha(img: DynamicImage) -> bool{
    dbg!(img.color().has_alpha());
    img.color().has_alpha()
}
// let coords = self.pos;
// let bytes = &self.data;
// let img_width = self.width;
// let img_height = self.height;
// let img_type = self.img_data_type.clone();
// dbg!(&img_type);
// // let cont = container.();
// let rect_pos = world_to_screen_2(coords, c_offset, c_scale);
// let rect_pos_2 = world_to_screen_2(Vector2::new(coords.x+img_width as f32, coords.y+img_height as f32), c_offset, c_scale);

// let to_draw_image = match graphics.create_image_from_raw_pixels(
//     img_type,
//     ImageSmoothingMode::NearestNeighbor,
//     Vector2::new(img_width, img_height),
//     &bytes,
// ){
//     Ok(img) => img,
//     Err(e) => panic!("Error creating image {}", e)
// };
// let rect = speedy2d::shape::Rectangle::new(
//     Vector2::new(rect_pos.x as f32, rect_pos.y as f32), 
//     Vector2::new(rect_pos_2.x as f32, rect_pos_2.y as f32)
// );

// graphics.draw_rectangle_image(rect, &to_draw_image);