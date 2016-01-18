extern crate sdl2;

#[macro_use]
mod events;

use sdl2::pixels::Color;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}

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

    let mut events = Events::new(sdl2_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
