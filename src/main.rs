use macroquad::{prelude::*, rand::rand};
use oorandom::Rand32;
use std::f64::consts::PI;

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
    struct Particle {
        x: u32,
        y: u32,
        vx: i32,
        vy: i32,
        age: i32,
    }
    let mut particles: Vec<Particle> = Vec::new();

    let mut image = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    // This function creates a new explosion at the specified screen co-ordinates
    pub fn explode(x: u32, y: u32, speed: u32, number_of_particles: u32) {
        let pi = std::f64::consts::PI;
        // these are new particles, so set their age to zero
        let mut age = 0;

        // generate 100 particles per explosion
        for _ in 0..number_of_particles as i32 {
            // for each particle generate a random angle and distance
            let angle = rand::gen_range(0, 2 * pi);
            let radius: f32 = rand::gen_range(0, 1).pow(0.5);

            // convert angle and distance from the explosion point into x and y velocity for the particle
            let vx: f32 = speed * radius * angle.sin();
            let vy: f32 = speed * radius * angle.cos();

            // add the particle's position, velocity, and age to the array
            particles.push((x, y, vx, vy, age));
        }
        
    }

    loop {
        clear_background(WHITE);

        let w = image.width() as u32;
        let h = image.height() as u32;
        // loop through all the particles in the array
        for (x, y) in particles {
            image.set_pixel(x, y, particle_color)
        }
        image.set_pixel(w / 2, h / 2, BLACK);

        texture.update(&image);
        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}