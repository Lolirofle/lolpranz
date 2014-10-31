extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::event;
use lolirofle::tdpg::object;
use lolirofle::tdpg::object::Position;
use lolirofle::tdpg::TdpgGame;
use std::num::Zero;

pub const JUMP_VELOCITY : f32 = 6.0;
pub const MOVE_MODIFIER : f32 = 0.3;
pub const GRAVITY       : f32 = 0.2;
pub const MAX_X_VELOCITY : f32 = 6.0;
pub const MAX_Y_VELOCITY : f32 = 10.0;

#[deriving(Clone)]
pub struct Player{
	position: Vector2<f32>,
	velocity: Vector2<f32>,
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),
		};
	}
}
impl object::Position for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl<'a> Updatable<TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : f64){
		self.position = self.position + self.velocity;
		self.velocity = self.velocity + Vector2(0.0,GRAVITY);
		let Vector2(_,pos_y) = self.position;
		let Vector2(vel_x,_) = self.velocity;
		if pos_y > 300.0 {
			self.velocity = Vector2(vel_x,0.0)
		}
	}
}
impl Renderable for Player{
	fn render(&self,renderer: &Renderer){
		renderer.render_rectangle(
			self.get_position(),
			Vector2(16.0 as f32,16.0)
		);
	}
}
impl object::Collision for Player {
	fn get_dimensions(&self) -> Vector2<f32> {
		Vector2::new(16f32, 32f32)
	}
}
impl object::Interactable for Player {}
impl EventHandler<event::Event> for Player{
	fn event(&mut self,e: event::Event){
		match e{
			event::Jump => {
				self.velocity = self.velocity-Vector2::new(0.0,JUMP_VELOCITY);
			},
			event::Move(v) => {
				self.velocity = self.velocity + v * MOVE_MODIFIER;
			},
			event::StopMove => {//TODO: Stops all movement, not only the player inflicted ones
				self.velocity = Zero::zero();
			},
			_ => {}
		}
	}
}
