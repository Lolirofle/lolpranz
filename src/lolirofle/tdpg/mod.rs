extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::game::Game;
use tdgl::lolirofle::gl::renderer::Renderer;
use gl;
use glfw;
use std::mem;

pub mod event;
pub mod object;
pub mod player;
pub mod wall;

pub struct TdpgGame<'a>{
	pub player        : player::Player,
	pub wall          : wall::Wall,
	pub renderables   : Vec<&'a mut Renderable + 'a>,
	pub updaters      : Vec<&'a mut Updatable<TdpgGame<'a>> + 'a>,
	pub event_handlers: Vec<&'a mut EventHandler<event::Event> + 'a>,

	pub gravity: f32,
	pub max_velocity: f32,
}

impl<'a> Game for TdpgGame<'a>{
	fn update(&mut self,delta_time: f64){
		unsafe{//TODO: How to fix efficiently
			let self2 = mem::transmute(&*self);
			self.player.update(self2,delta_time);
		}
	}

	fn render(&self,renderer: &Renderer){
		gl::Clear(gl::COLOR_BUFFER_BIT);

		self.player.render(renderer);
		self.wall.render(renderer);
	}

	fn event(&mut self,window:&mut glfw::Window,event:glfw::WindowEvent) {
		match match event{
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
				window.set_should_close(true);
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(event::Jump),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(event::Move(-1.0)),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(event::Move( 1.0)),
			
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::StopMove),
			_ => None
		}{
			Some(e) => {self.player.event(e);},
			None    => {}
		};
	}

	fn init() -> TdpgGame<'a>{
		return TdpgGame{
			player: player::Player::new(),
			wall  : wall::Wall::new(Vector2::new(50.0,240.0),Vector2::new(16f32,16f32)),
			renderables   : Vec::with_capacity(20u),
			updaters      : Vec::with_capacity(20u),
			event_handlers: Vec::with_capacity(20u),

			gravity       : 0.2,
			max_velocity  : 8.0,
		};
	}
}
