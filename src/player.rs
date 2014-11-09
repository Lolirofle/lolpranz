extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::{Update,Render,EventHandler};
use tdgl::graphics::renderer::Renderer;
use std::num::Zero;
use std::time::Duration;

use game::TdpgGame;
use event;
use object;
use object::Position;

pub const JUMP_VELOCITY : f32 = 6.0;
pub const MOVE_MODIFIER : f32 = 0.3;
pub const GRAVITY       : f32 = 0.2;
pub const MAX_X_VELOCITY : f32 = 6.0;
pub const MAX_Y_VELOCITY : f32 = 10.0;

#[deriving(Clone)]
pub struct Player{
	position: Vector<f32>,
	velocity: Vector<f32>,
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector{x: 0.0,y: 0.0},
			velocity: Vector{x: 0.0,y: 0.0},
		};
	}
}
impl object::Position for Player{
	fn get_position(&self) -> Vector<f32>{
		return self.position;
	}
}
impl<'a> Update<&'a TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : Duration){
		self.position = self.position + self.velocity;
		self.velocity.y += GRAVITY;
		if self.position.y > 300.0 {
			self.velocity.y = 0.0;
		}
	}
}
impl Render<()> for Player{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		renderer.render_rectangle(
			self.get_position(),
			Vector{x: 16.0 as f32,y: 16.0}
		);
	}
}
impl object::Collision for Player {
	fn get_dimensions(&self) -> Vector<f32> {
		Vector{x: 16f32,y:  32f32}
	}
}
impl object::Interactable for Player {}
impl EventHandler<event::Event> for Player{
	fn event(&mut self,e: event::Event){
		match e{
			event::Jump => {
				self.velocity = self.velocity-Vector{x: 0.0,y: JUMP_VELOCITY};
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
