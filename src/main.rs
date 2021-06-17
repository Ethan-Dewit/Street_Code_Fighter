extern crate sdl2;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread;
use std::fs;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod characters; // for characterAbstract
pub mod view; // for core
pub mod input; // for inputHandler and movement
pub mod animation;
use crate::view::core; // need for SDLCore
use crate::view::core::Demo; // need for SDLCore's Demo

const TITLE: &str = "Street Code Fighter";
const TIMEOUT: u64 = 5000;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

// SDL structure
pub struct SDL {
    core: core::SDLCore,
}

impl core::Demo for SDL {
    fn init() -> Result<Self, String> {
        let core = core::SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(SDL{ core })
    }

    fn run(&mut self) -> Result<(), String> {

        // creating initial character state and 
        let cs = characters::characterAbstract::CharacterState::new();
        let mut fighter = characters::characterAbstract::Fighter::new(cs);

        // loading textures
        let texture_creator = self.core.wincan.texture_creator();
        let texture = texture_creator.load_texture("src/assets/images/characters/python/fjump-outline.png")?; // TODO: organize into hashmaps
        
        // game loop
        'gameloop: loop {

            for event in self.core.event_pump.poll_iter() { //input events
                match event {
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                    _ => { input::inputHandler::keyboard_input(&event, &mut fighter); } // handles non-exit behavior
                } // end match
            } // end for loop

            // updates in game ... 

            // render canvas
            Self::render(&mut self.core.wincan, Color::RGB(222,222,222), &texture, &fighter);

        } // close gameloop

        Ok(()) // needs to return Result :)

    } // close run fn

    fn render(canvas: &mut WindowCanvas,
              color: Color,
              texture: &Texture,
              fighter: &characters::characterAbstract::Fighter,
              ) -> Result<(), String> {

            // color
            canvas.set_draw_color(color);
            canvas.clear();

            // set canvas height
            let (width, height) = canvas.output_size()?;


            let (frame_width, frame_height) = fighter.char_state.sprite.size();

            let current_frame = Rect::new(
                fighter.char_state.sprite.x() + frame_width as i32 * fighter.char_state.current_frame,
                fighter.char_state.sprite.y(), // should always be 0, since y should remain consistent
                frame_width,
                frame_height,
            );

            // (0, 0) coordinate = center of the scren
            let screen_position = fighter.char_state.position + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
            canvas.copy(texture, current_frame, screen_rect)?;

            canvas.present();

            Ok(())
    }
}

// // run credits
// pub fn run_credits() -> Result<(), String> {

//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;
//     let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
//     let window = video_subsystem
//         .window("Fighter Game Team", CAM_W, CAM_H)
//         .build()
//         .map_err(|e| e.to_string())?;

//     let canvas = window
//         .into_canvas()
//         .accelerated();

//     let canvas = canvas.present_vsync();

//     let mut canvas = canvas
//         .build()
//         .map_err(|e| e.to_string())?;

//     let texture_creator = canvas.texture_creator();

//     //init and fill a vector with our textures
//     let paths = fs::read_dir("./src/assets/images/credits").unwrap();
//     let mut textures: Vec<sdl2::render::Texture> = Vec::new();

//     for path in paths {
//             textures.push(texture_creator.load_texture(path.unwrap().path().display().to_string())?);
//     }

//     canvas.set_draw_color(Color::RGBA(0, 128, 128, 255));
//     canvas.clear();

//     /* loop to display each texture, making sure they fit within the 
//         window and are positioned at the center before displaying them */
//     for t in textures {
//         let mut img_h = t.query().height;
//         let mut img_w = t.query().width;

//         while img_h > CAM_H || img_w > CAM_W {
//             img_h = img_h/2;
//             img_w = img_w/2;
//         }

//         let center_x = (CAM_W/2) - (img_w/2);
//         let center_y = (CAM_H/2) - (img_h/2);
//         let display_area = Rect::new(center_x as i32, center_y as i32, img_w, img_h);

//         canvas.copy(&t, None, display_area)?;
//         canvas.present();

//         thread::sleep(Duration::from_millis(TIMEOUT));

//         canvas.clear();
//     }

//     Ok(())
// }

fn main() -> Result<(), String> {
    core::runner(TITLE, SDL::init); // run game
    // run_credits()?;
    
    Ok(())
}

