extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::*;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::event;
use lolirofle::tdpg::object::{Position,Velocity,Collision,Interactable};
use lolirofle::tdpg::TdpgGame;

pub const MOVE_VELOCITY : f32 = 1.5;

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
impl Velocity for Player{
	fn get_velocity(&self) -> Vector2<f32>{
		return self.velocity;
	}
}
impl<'a> Updatable<TdpgGame<'a>> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : f64){
		self.velocity = self.velocity + Vector2(0.0,game.gravity);
		self.velocity.limit_magnitude(game.max_velocity);
		self.position = self.position + self.velocity;
		match self.collision_check(game.wall){
			Some(Vector2(gap_x,gap_y)) => {
				let Vector2(ref mut pos_x,ref mut pos_y) = self.position;
				let Vector2(ref mut vel_x,ref mut vel_y) = self.velocity;

				if *vel_x!=0.0 && gap_x>0.0{
					*pos_x -= gap_x * vel_x.signum();
					*vel_x = 0.0;
				}

				if *vel_y!=0.0 && gap_y>0.0{
					*pos_y -= gap_y * vel_y.signum();
					*vel_y = 0.0;
				}
			},
			None => {}
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
			event::Move(vel_x) => {
				self.velocity = self.velocity + Vector2(vel_x * MOVE_VELOCITY,0.0);
			},
			event::StopMove => {//TODO: Stops all movement, not only the player inflicted ones, or should the current "velocity" field just store player inflicted velocity? Else separate it.
				let Vector2(ref mut vel_x,_) = self.velocity;
				*vel_x = 0.0;
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
