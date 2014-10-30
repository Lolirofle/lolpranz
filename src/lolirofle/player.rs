use lolirofle::game::gameloop::*;
use lolirofle::gl::renderer::Renderer;
use lolirofle::object::{Collision, Interactable, Wall, Position};
use lolirofle::tdpg::TdpgGame;
use lolirofle::vector::Vector2;

#[deriving(Clone)]
pub struct Player{
	position: Vector2<f32>,
	velocity: Vector2<f32>
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),
		};
	}
}
impl Position for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl Updatable<TdpgGame> for Player{
	fn update(&mut self,game: &TdpgGame,delta_time : f64){
		self.position = self.position + self.velocity;
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
impl Collision for Player {
    fn get_dimensions(&self) -> Vector2<f32> {
        Vector2::new(16f32, 32f32)
    }
}
impl Interactable for Player {}
impl EventHandler for Player{
	fn event(&mut self,e: Event){
		match e{
			Jump(f) => {
				self.velocity = self.velocity-Vector2::new(0.0,f);
			},
			Move(v) => {
				self.velocity = self.velocity+v;
			},
		}
	}
}
