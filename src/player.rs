extern crate "2dgl"as tdgl;

use tdgl::data::vector2::Vector as VectorTrait;
use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::{Update,Render,EventHandler};
use tdgl::graphics::renderer::Renderer;
use std::time::Duration;

use event;
use object::{Position,Velocity,Dimension,Interact};
use game::TdpgGame;

pub const MOVE_VELOCITY : f32 = 1.5;

#[deriving(Clone)]
pub struct Player{
	position: Vector<f32>,
	velocity: Vector<f32>,

	//Constants
	jump_velocity: f32,
	move_acceleration: f32,

	//States
	move_velocity: f32,
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector{x: 0.0,y: 0.0},
			velocity: Vector{x: 0.0,y: 0.0},

			jump_velocity: 6.0,
			move_velocity: 0.0,
			move_acceleration: 0.25,
		};
	}
}
impl Position for Player{
	fn get_position(&self) -> Vector<f32>{
		return self.position;
	}
}
impl Velocity for Player{
	fn get_velocity(&self) -> Vector<f32>{
		return self.velocity;
	}
}
impl<'a> Update<&'a TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,_ : Duration){
		//TODO: Optimize everything, including finding better methods for doing these things. At least it's working now

		//Gravity affecting velocity
		self.velocity.y += game.gravity;

		{//Movement affecting velocity //TODO: Turning around while moving makes friction apply to the other direction immediately
			if self.velocity.x.abs() < self.move_velocity.abs(){
				self.velocity.x = (self.move_acceleration + self.velocity.x.abs()).min(self.move_velocity.abs())*self.move_velocity.signum();
				//self.velocity.x = self.move_velocity;
			}
		}

		//Friction affecting velocity
		let friction = self.velocity.unit()/16.0;
		if friction.magnitude() < self.velocity.magnitude(){
			self.velocity = self.velocity - friction;//.limit_magnitude(game.max_velocity);
		}

		//Velocity affecting position
		self.position = self.position + self.velocity;

		//Collision checking
		match self.collision_check(&game.wall){
			Some(gap) => {
				if gap.x>0.0 && gap.x<=gap.y{
					self.position.x -= gap.x * self.velocity.x.signum();
					self.velocity.x /= -2.0;
				}

				if gap.y>0.0 && gap.y<=gap.x{
					self.position.y -= gap.y * self.velocity.y.signum();
					self.velocity.y /= -2.0;
				}
			},
			None => {}
		}
	}
}

impl Dimension for Player{
	fn get_dimensions(&self) -> Vector<f32> {
		Vector{x: 16.0,y: 32.0}
	}
}

impl Render<()> for Player{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions()
		);
	}
}

impl Interact for Player {}

impl EventHandler<event::Event> for Player{
	fn event(&mut self,e: event::Event){
		match e{
			event::Jump => {
				self.velocity.y -= self.jump_velocity;
			},
			event::Move(vel_x) => {
				self.move_velocity = vel_x*MOVE_VELOCITY;
			},
			_ => {}
		}
	}
}

//fn towards_zero<T>(a: &T,b: &T) -> T where T: std::num::Signed{
/*fn towards_zero(a: f32,b: f32) -> f32{//TODO: Move this function to a more appropriate place
	let b2 = b * a.signum();
	if a.abs()>b{
		a - b2
	}else{
		0.0
	}
}
#[test]
fn test_towards_zero(){
	assert!(towards_zero( 3.0,2.0)== 1.0);
	assert!(towards_zero(-3.0,2.0)==-1.0);

	assert!(towards_zero( 3.0,3.0)== 0.0);
	assert!(towards_zero(-3.0,3.0)== 0.0);

	assert!(towards_zero( 3.0,4.0)== 0.0);
	assert!(towards_zero(-3.0,4.0)== 0.0);
}
*/
