use core::cmp::partial_min;
use core::num::Float;
use num::{Signed,Zero};
use std::time::Duration;
use tdgl::data::two_dim::vector;
use tdgl::data::two_dim::vector::Vector;
use tdgl::game::gameloop::{Update,Render};
use tdgl::graphics::renderer::Renderer;

use event;
use game::TdpgGame;
use object::{Position,Velocity,Dimension,Interact};

pub const MOVE_VELOCITY : f32 = 1.5;

pub struct Player{
	player_id: u8,
	event_receiver: Receiver<event::Game>,

	position: vector::Coord<f32>,
	velocity: vector::Coord<f32>,

	//Constants
	jump_velocity: f32,
	move_acceleration: f32,

	//States
	move_velocity: f32,
}
impl Player{
	pub fn new(player_id: u8,position: vector::Coord<f32>) -> (Player,Sender<event::Game>){
		let (transmitter,receiver) = channel();
		return (Player{
			event_receiver: receiver,
			player_id: player_id,

			position: position,
			velocity: vector::Coord{x: 0.0,y: 0.0},

			jump_velocity: 6.0,
			move_velocity: 0.0,
			move_acceleration: 0.25,
		},transmitter);
	}
}
impl Position for Player{
	fn get_position(&self) -> vector::Coord<f32>{
		return self.position;
	}
}
impl Velocity for Player{
	fn get_velocity(&self) -> vector::Coord<f32>{
		return self.velocity;
	}
}

impl<'a> Update<(u32,&'a TdpgGame<'a>)> for Player{
	fn update(&mut self,(id,game): (u32,&TdpgGame),_ : Duration){
		//Handle events
		while let Ok(e) = self.event_receiver.try_recv(){
			match e{
				event::Game::Player(player_id,pe) => if self.player_id == player_id{match pe{
					event::Player::Jump => {
						self.velocity.y -= self.jump_velocity;//TODO: `should_jump` because of collision checking for if standing on something
					},
					event::Player::Move(vel_x) => {
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
				self.velocity.x = partial_min(
					self.move_acceleration + self.velocity.x.abs(),
					(self.move_velocity.abs())*self.move_velocity.signum()
				).unwrap();//TODO: May crash
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
	fn get_dimensions(&self) -> vector::Coord<f32> {
		vector::Coord{x: 16.0,y: 32.0}
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

//fn towards_zero<T>(a: &T,b: &T) -> T where T: core::num::Signed{
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
