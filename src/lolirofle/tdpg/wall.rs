extern crate "2dgl"as tdgl;

use tdgl::lolirofle::data::vector::Vector2;
use tdgl::lolirofle::game::gameloop::Render;
use tdgl::lolirofle::gl::renderer::Renderer;
use lolirofle::tdpg::object::{Position,Dimension,Interact};

pub struct Wall {
	pos      : Vector2<f32>,
	dimension: Vector2<f32>,
}

impl Wall {
	pub fn new(p : Vector2<f32>, d : Vector2<f32>) -> Wall {
		Wall {pos : p, dimension : d}
	}
}

impl Position for Wall {
	fn get_position(&self) -> Vector2<f32> { self.pos }
}

impl Dimension for Wall {
	fn get_dimensions(&self) -> Vector2<f32> { self.dimension }
}

impl Render for Wall {
	fn render(&self, renderer : &Renderer) {
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions(),
		)
	}
}

impl Interact for Wall {}
