/* Minimal example to showcase the Rust SDL2 bindings working with an emscripten target (asmjs,wasm32).
 * Install one or both of the following Rust target triples:
 *   rustup target add wasm32-unknown-emscripten
 *   rustup target add asmjs-unknown-emscripten
 *
 * Build:
 *   source emsdk/emsdk_env.sh 
 *   cd src/
 *   em++ -c gxx_personality_v0_stub.cpp
 *   cargo build --target=asmjs-unknown-emscripten --release
 *   cargo build --target=wasm32-unknown-emscripten --release
 *
 * Start a web server and run the example:
 *   emrun index-asmjs.html 
 *   (or emrun index-wasm.html)
 */
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use std::convert::TryInto;

fn main() -> Result<(), String> {

    #[cfg(target_os = "emscripten")]
    let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?; 
    let (width,height) = (640,480);
    let window = video_subsystem
        .window("RuSDLem", width, height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();

    //setup the background image, light grey with a blue diagonal line from upper right to lower left
    let mut bg_texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, width, height)
        .map_err(|e| e.to_string())?;

    canvas.with_texture_canvas(&mut bg_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(230,230,230,255));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(0,0,255,255));
            { 
                let w:i32 = width.try_into().unwrap();
                let h:i32 = height.try_into().unwrap();
                texture_canvas.draw_line(Point::new(w-1,0),
                                         Point::new(0,h-1)).unwrap();
            }
        }).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        let event = event_pump.wait_event(); //blocking wait for events
        
        canvas.copy(&bg_texture, None, None).unwrap();
                
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape),..} | Event::Quit { .. } 
                => { break 'mainloop; }
            Event::KeyDown {keycode: Some(Keycode::F),..}
                => { 
                    //"F" -> full screen mode
                    canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True)?;
                }
            Event::MouseMotion {x, y, .. } => {
                //draw a red line from the upper left corner to the current mouse position
                canvas.set_draw_color(Color::RGBA(255,0,0,255));
                canvas.draw_line(Point::new(0,0), Point::new(x,y)).unwrap();
                ()}
            _ => {
                println!("{:?}",event); //Print out other events to the "console"
                ()}
        }
        canvas.present();
    };
    
    Ok(())
}
