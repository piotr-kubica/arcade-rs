extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;

fn main() {
    // Initialize sdl2
    let sdl2_context = sdl2::init().unwrap();
    let video = sdl2_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    // Render a fully black window
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    thread::sleep_ms(3000);
}
