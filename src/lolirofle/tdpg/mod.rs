extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::game::Game;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::object::Interact;

use gl;
use glfw;
use std::mem;
use std::boxed::HEAP;

pub mod event;
pub mod object;
pub mod player;
pub mod wall;
pub mod dummyhandler;

pub struct TdpgGame<'a>{
	pub renderables   : Vec<Box<Render + 'a>>,
	pub updatables    : Vec<Box<Update<TdpgGame<'a>> + 'a>>,
	pub event_handlers: Vec<Box<EventHandler<event::Event> + 'a>>,
    pub interactables : Vec<Box<Interact + 'a>>,

	pub gravity: f32,
	pub max_velocity: f32,
}

impl<'a> Game for TdpgGame<'a>{
	fn update(&mut self,delta_time: f64){
		unsafe{//TODO: How to fix efficiently
			let self2 = mem::transmute(&*self);
			for obj in self.updatables.iter_mut() {
                obj.update(self2, delta_time);
            }
        }
    }

	fn render(&self,renderer: &Renderer){
		gl::Clear(gl::COLOR_BUFFER_BIT);

		for renderable in self.renderables.iter() {
            renderable.render(renderer);
        }
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
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::Move(0.0)),
			_ => None
		}{
			Some(e) => {
                for handler in self.event_handlers.iter_mut() {
                    handler.event(e);
                }
            },
			None    => {}
		};
	}

	fn init() -> TdpgGame<'a>{
        use lolirofle::tdpg::object::Interact;
		let mut game = TdpgGame {
			renderables   : Vec::with_capacity(20u),
			updatables    : Vec::with_capacity(20u),
			event_handlers: Vec::with_capacity(20u),
            interactables : Vec::with_capacity(20u),

			gravity       : 0.2,
			max_velocity  : 8.0,
		};
        let mut handler = dummyhandler::DummyHandler::new();
        game.event_handlers.push(box(HEAP) handler);
        let player = box player::Player::new();
        let wall   = box wall::Wall::new(Vector2::new(50.0,240.0),Vector2::new(320.0,16.0));
        game.event_handlers.push(player);
        game.renderables.push(player);
        game.updatables.push(player);
        //game.interactables.push(player); Player doesn't collide with itself. But if we're having
        //multiple players we should do a different workaround
        game.renderables.push(wall);
        game.interactables.push(wall);
        game
	}
}
