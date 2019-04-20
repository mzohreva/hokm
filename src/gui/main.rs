
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use super::*;

pub fn gui_main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");

    let window = video_subsystem
        .window("Hokm", SCENE_WIDTH, SCENE_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let textures = Textures::load(&texture_creator)?;
    let mut scene = Scene::new();
    scene.paint(&textures, &mut canvas)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut paused = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..}
                    => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), ..}
                    => paused = !paused,
                Event::MouseButtonDown { x, y, .. } => {
                    scene.click(x, y);
                    scene.paint(&textures, &mut canvas)?;
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        if !paused && scene.process() {
            scene.paint(&textures, &mut canvas)?;
        }
    }

    Ok(())
}
