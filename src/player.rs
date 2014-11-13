extern crate "2dgl"as tdgl;

use core::num::Zero;
use tdgl::data::vector2::Vector as VectorTrait;
use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::{Update,Render};
use tdgl::graphics::renderer::Renderer;
use std::time::Duration;

use event;
use object::{Position,Velocity,Dimension,Interact};
use game::TdpgGame;

pub const MOVE_VELOCITY : f32 = 1.5;

pub struct Player{
	player_id: u8,

	position: Vector<f32>,
	velocity: Vector<f32>,

	//Constants
	jump_velocity: f32,
	move_acceleration: f32,

	//States
	move_velocity: f32,
	
	event_receiver: Receiver<event::Event>,
}
impl Player{
	pub fn new(player_id: u8,position: Vector<f32>) -> (Player,Sender<event::Event>){
		let (transmitter,receiver) = channel();
		return (Player{
			event_receiver: receiver,
			player_id: player_id,

			position: position,
			velocity: Vector{x: 0.0,y: 0.0},

			jump_velocity: 6.0,
			move_velocity: 0.0,
			move_acceleration: 0.25,
		},transmitter);
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
impl<'a> Update<(u32,&'a TdpgGame<'a>)> for Player{
	fn update(&mut self,(id,game): (u32,&TdpgGame),_ : Duration){
		//Handle events
		while let Ok(e) = self.event_receiver.try_recv(){
			match e{
				event::Player(player_id,pe) => if self.player_id == player_id{match pe{
					event::Jump => {
						self.velocity.y -= self.jump_velocity;//TODO: `should_jump` because of collision checking for if standing on something
					},
					event::Move(vel_x) => {
						self.move_velocity = vel_x*MOVE_VELOCITY;
					},
					_ => {}
				}},
				//_=> {}
			}
		}

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
		}else{
			self.velocity = Zero::zero();
		}

		//Velocity affecting position
		self.position = self.position + self.velocity;

		//Collision checking
		for (&obj_id,obj) in game.interactables.iter(){
			if id != obj_id && obj.is_solid(self){
				match self.collision_check(*obj){
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

impl Interact for Player{
	fn is_solid(&self,_: &Interact) -> bool{true}
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
