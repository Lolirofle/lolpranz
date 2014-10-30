use lolirofle::player::Player;
use lolirofle::game::gameloop::*;
use lolirofle::gl::renderer::Renderer;
use lolirofle::game::Game;
use lolirofle::vector::Vector2;
use lolirofle::object::Wall;
use gl;
use glfw;
use std::mem;

#[deriving(Clone)]
pub struct TdpgGame{
	player: Player,
    wall  : Wall
}
impl Game for TdpgGame{
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
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_) => Some(Jump(20.0*16.0)),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_) => Some(Move(Vector2::new(-10.0*16.0,0.0))),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_) => Some(Move(Vector2::new( 10.0*16.0,0.0))),
			_ => None
		}{
			Some(e) => {self.player.event(e);},
			None    => {}
		};
	}

	fn init() -> TdpgGame{
		return TdpgGame{
			player: Player::new(),
            Wall  : Wall::new(Vector2::new(50f32,240),Vector2::new(16f32,16f32)),
		};
	}
}
