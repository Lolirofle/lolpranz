extern crate "2dgl"as tdgl;

use tdgl::data::vector::Vector2;
use tdgl::game::gameloop::*;
use tdgl::game::Game;
use tdgl::graphics::renderer::Renderer;
use gl;
use glfw;
use std::mem;

use event;
use object;
use player;
use wall;

pub struct TdpgGame<'a,R>
	where R: Renderer
{
	player        : player::Player,
	wall          : wall::Wall,
	renderables   : Vec<&'a mut Renderable + 'a>,
	updaters      : Vec<&'a mut Updatable<TdpgGame<'a,R>> + 'a>,
	event_handlers: Vec<&'a mut EventHandler<event::Event> + 'a>,
}

impl<'a,R: Renderer> Game<R> for TdpgGame<'a,R>{
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

	fn event(&mut self,event: event::Event){
		self.player.event(e);
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
