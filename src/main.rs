use indicatif::ProgressBar;
use std::time::Duration;
use image::{RgbImage, ImageBuffer, Rgb};
mod dspace;
use dspace::*;
mod ray;
use ray::Ray;

fn main() {
    
    //Our PNG dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    let camera_center = Point3d::new(0.0, 0.0, 0.0);

    //Our camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;

    //Calculating vectors across horizontal and down vertical viewport edges
    let viewport_h = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    //Calculate horizontal and vertical delta vectors from origin, pixel by pixel
    let lower_left = camera_center - viewport_h / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Our buffer set to our images dimensions
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    //Spinning progress bar to show we're rendering (currently too fast to show anything)
    let bar = ProgressBar::new_spinner();

    //Sets our spin to once every 100 ms
    bar.enable_steady_tick(Duration::from_millis(100));

    //For loop given our x and y coordinates of image, pixels being our current pixel we're on
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){

        let h = x as f64 / (WIDTH - 1) as f64;
        let v = 1.0 - (y as f64 / (HEIGHT - 1) as f64);
        let ray = Ray::new(camera_center, lower_left + h * viewport_h + v * viewport_v - camera_center);    

        let display = ray.color().to_rgb();
        *pixel = Rgb(display);
    }

    //Finish our progress bar after our for loop
    bar.finish();

    //Switch statement to either write to png file or throw an error
    match buffer.save("image.png") {
        Err(e) => eprintln!("Unable to write to file: {}", e),
        Ok(()) => println!("Render finished successfully."),
    };
}