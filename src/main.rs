use macroquad::prelude::*;

#[macroquad::main("Explosion Demo")]
async fn main() {
    let screen_width = 800 as f32;
    let screen_height = 600 as f32;
    request_new_screen_size(screen_width, screen_height);

    let mut image = Image::gen_image_color(screen_width as u16, screen_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);
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