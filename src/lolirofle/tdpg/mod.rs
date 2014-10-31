use lolirofle::data::vector::Vector2;
use lolirofle::game::gameloop::*;
use lolirofle::game::Game;
use lolirofle::gl::renderer::Renderer;
use gl;
use glfw;
use std::mem;

pub mod object;
pub mod player;
pub mod wall;

pub struct TdpgGame<'a>{
	player        : player::Player,
	wall          : wall::Wall,
	renderables   : Vec<&'a mut Renderable + 'a>,
	updaters      : Vec<&'a mut Updatable<TdpgGame<'a>> + 'a>,
	event_handlers: Vec<&'a mut EventHandler + 'a>,
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
	}

	fn event(&mut self,window:&mut glfw::Window,event:glfw::WindowEvent) {
		match match event{
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
				window.set_should_close(true);
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(Jump),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(Move(Vector2::new(-1.0,0.0))),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(Move(Vector2::new( 1.0,0.0))),
			
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(StopMove),
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
		};
	}
}
