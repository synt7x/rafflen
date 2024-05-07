#![windows_subsystem = "windows"]

pub mod dancers;
pub mod renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rafflen", 1024, 600).position_centered().build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("assets/cozette.ttf", 16)?;
    let texture_context = canvas.texture_creator();
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut context = renderer::RenderContext {
        canvas: canvas,
        texture: texture_context,
        font: font,
        waiting: true,
        frame: 0,
    };

    let texture_binding = context.canvas.texture_creator();
    let mut textures = renderer::RenderTextures::new(&texture_binding);

    let mut dancers: Vec<dancers::Dancer> = Vec::new();
    let mut delta_time = 0.0;

    for i in 0..100 {
        let name = format!("Dancer {}", i);
        let dancer = dancers::Dancer::new(&mut context.font, &texture_binding, name);
        dancers.push(dancer);
    }

    loop {
        let now = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(());
                },
                Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                    context.waiting = false;
                },
                _ => {}
            }
        }

        context.canvas.clear();
        renderer::render(&mut context, &mut dancers, &mut textures, delta_time);
        context.canvas.present();

        //::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        delta_time = now.elapsed().as_secs_f64() / 60.0;

    }
}