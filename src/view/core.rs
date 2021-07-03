extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use std::collections::HashMap;
use sdl2::video::WindowContext;
use sdl2::rect::Point;

use crate::characters;
use crate::animation;
use crate::physics;

pub struct SDLCore{
	sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
}

impl SDLCore{
	pub fn init(
		title: &str,
		vsync: bool,
		width: u32,
		height: u32,
	) -> Result<SDLCore, String>{
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;


		let window = video_subsys.window(title, width, height).build().map_err(|e| e.to_string())?;
		let wincan = window.into_canvas().accelerated();

		let wincan = if vsync {
			wincan.present_vsync()
		}else{
			wincan
		};

		let mut wincan = wincan.build().map_err(|e| e.to_string())?;

		let event_pump = sdl_cxt.event_pump()?;

		wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		wincan.clear();
		wincan.present();

		Ok(SDLCore{
			sdl_cxt,
			wincan,
			event_pump,
		})
	}

	pub fn render(&mut self,
				color: Color,
				texture: &Texture,
				fighter: &characters::characterAbstract::Fighter,
				hazard: &physics::hazard::Hazard,
				hazard_texture: &Texture
				) -> Result<(), String>{

		// color
		self.wincan.set_draw_color(color);
		self.wincan.clear();

		// set canvas height
		let (width, height) = self.wincan.output_size()?;

		let (frame_width, frame_height) = fighter.char_state.sprite.size();

		//get curent chararcter state
        let current_frame = Rect::new(
        	//determins which sprite to get, using current_frame as offset on sprite sheet
            fighter.char_state.sprite.x() + frame_width as i32 * fighter.char_state.current_frame,
            fighter.char_state.sprite.y(), // should always be 0, since y should remain consistent
            frame_width,
            frame_height,
        );
		let hazard_frame = Rect::new(0, 0, 100, 100);

        // (0, 0) cordinate = center of the scren
		// make new rect and screen pos //
        let screen_position = fighter.char_state.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);

		// hazard rectangle & position
		let hazard_screen_position = hazard.position;
		let hazard_screen_rectangle = hazard.sprite;

		// copy textures
        self.wincan.copy(texture, current_frame, screen_rect)?;
		self.wincan.copy(hazard_texture, hazard_frame, hazard_screen_rectangle)?;
        self.wincan.present();

        /*
        println!("Frame count is: {}    Frame Per State is: {}    Current Sprite is: {}    State is: {:?}",
        fighter.char_state.frame_count, fighter.char_state.frames_per_state, 
        fighter.char_state.current_frame, fighter.char_state.state);
		*/


        Ok(())
	} // closing render fun
/*
    // NOT FUNCTIONING YET
    fn load_textures(texture_creator: &'t TextureCreator<WindowContext>,
                     f: &mut characters::characterAbstract::Fighter) {

            // let idle = texture_creator.load_texture("src/assets/images/characters/python/idle-outline.png");

            // match idle {
            //     Ok(i) =>  { f.add_texture(animation::sprites::State::Idle, i); },
            //     Err(e) => { panic!("Nooo"); },
            // }

    } // close load_textures
*/

}
