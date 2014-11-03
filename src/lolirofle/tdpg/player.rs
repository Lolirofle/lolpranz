extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::event;
use lolirofle::tdpg::object::{Position,Collision,Interactable};
use lolirofle::tdpg::TdpgGame;
use std::num::Zero;

pub const MOVE_MODIFIER : f32 = 0.3;

#[deriving(Clone)]
pub struct Player{
	position: Vector2<f32>,
	velocity: Vector2<f32>,

	jump_velocity: f32,
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),

			jump_velocity: 6.0,
		};
	}
}
impl Position for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl<'a> Updatable<TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : f64){
		self.position = self.position + self.velocity;
		self.velocity = self.velocity + Vector2(0.0,game.gravity);
		let Vector2(_,pos_y) = self.position;
		let Vector2(vel_x,_) = self.velocity;
		if pos_y > 300.0 {//TODO: Hardcoded floor?
			self.velocity = Vector2(vel_x,0.0)
		}
	}
}

impl Collision for Player{
	fn get_dimensions(&self) -> Vector2<f32> {
		Vector2::new(16.0,32.0)
	}
}

impl Renderable for Player{
	fn render(&self,renderer: &Renderer){
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions()
		);
	}
}

impl Interactable for Player {}
impl EventHandler<event::Event> for Player{
	fn event(&mut self,e: event::Event){
		match e{
			event::Jump => {
				self.velocity = self.velocity-Vector2::new(0.0,self.jump_velocity);
			},
			event::Move(v) => {
				self.velocity = self.velocity + v * MOVE_MODIFIER;
			},
			event::StopMove => {//TODO: Stops all movement, not only the player inflicted ones, or should the current "velocity" field just store player inflicted velocity? Else separate it.
				self.velocity = Zero::zero();
			},
			_ => {}
		}
	}
}
