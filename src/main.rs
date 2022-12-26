use macroquad::prelude::*;

#[macroquad::main("Explosion Demo")]
async fn main() {
    let screen_width = 800 as f32;
    let screen_height = 600 as f32;
    request_new_screen_size(screen_width, screen_height);

    // how much a particle slows down by each second
    let drag = 0.8 as f32;
    // the color of each particle in R,G,B values
    let particle_color = Color::new(255.0, 230.0, 128.0, 1.0);
    // the time in seconds for which a particle is displayed
    let max_age = 3;
    // particle speed
    let speed = 300 as u32;
    // an array to hold the details of the explosions particles on the screen
    let mut particles : Vec<u16> = Vec::new();

    let mut image = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    // This function creates a new explosion at the specified screen co-ordinates
    pub fn explode(x: u32, y: u32, speed: u32) {
        // these are new particles, so set their age to zero
        let mut age = 0;
        
    }

    loop {
        clear_background(WHITE);

        let w = image.width() as u32;
        let h = image.height() as u32;

        image.set_pixel(w / 2, h / 2, BLACK);

        texture.update(&image);
        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}