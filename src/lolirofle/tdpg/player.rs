extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::event;
use lolirofle::tdpg::object::{Position,Velocity,Dimension,Interact};
use lolirofle::tdpg::TdpgGame;

pub const MOVE_VELOCITY : f32 = 1.5;

#[deriving(Clone)]
pub struct Player{
	position: Vector2<f32>,
	velocity: Vector2<f32>,

	//Constants
	jump_velocity: f32,
	move_acceleration: f32,

	//States
	move_velocity: f32,
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),

			jump_velocity: 6.0,
			move_velocity: 0.0,
			move_acceleration: 0.25,
		};
	}
}
impl Position for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl Velocity for Player{
	fn get_velocity(&self) -> Vector2<f32>{
		return self.velocity;
	}
}
impl<'a> Update<TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : f64){
		//TODO: Optimize everything, including finding better methods for doing these things. At least it's working now

		//Gravity affecting velocity
		self.velocity = self.velocity + Vector2(0.0,game.gravity);

		{//Movement affecting velocity //TODO: Turning around while moving makes friction apply to the other direction immediately
			let Vector2(ref mut vel_x,_) = self.velocity;
			if (*vel_x).abs() < self.move_velocity.abs(){
				*vel_x = (self.move_acceleration + vel_x.abs()).min(self.move_velocity.abs())*self.move_velocity.signum();
				//*vel_x = self.move_velocity;
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
        for ref wall in game.interactables.iter() {
	        match self.collision_check(**wall){
		    	Some(Vector2(gap_x,gap_y)) => {
		    		let Vector2(ref mut pos_x,ref mut pos_y) = self.position;
		    		let Vector2(ref mut vel_x,ref mut vel_y) = self.velocity;

		    		if gap_x>0.0 && gap_x<=gap_y{
		    			*pos_x -= gap_x * vel_x.signum();
		    			*vel_x /= -2.0;
		    		}

		    		if gap_y>0.0 && gap_y<=gap_x{
		    			*pos_y -= gap_y * vel_y.signum();
		    			*vel_y /= -2.0;
		    		}
		    	},
		    	None => {}
		    }
        }
	}
}

impl Dimension for Player{
	fn get_dimensions(&self) -> Vector2<f32> {
		Vector2::new(16.0,32.0)
	}
}

impl Render for Player{
	fn render(&self,renderer: &Renderer){
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
				self.velocity = self.velocity-Vector2::new(0.0,self.jump_velocity);
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
