extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::Render;
use tdgl::graphics::renderer::Renderer;

use object::{Position,Dimension,Interact};

pub struct Wall {
	pos      : Vector<f32>,
	dimension: Vector<f32>,
}

impl Wall {
	pub fn new(p : Vector<f32>, d : Vector<f32>) -> Wall {
		Wall {pos : p, dimension : d}
	}
}

impl Position for Wall {
	fn get_position(&self) -> Vector<f32> { self.pos }
}

impl Dimension for Wall {
	fn get_dimensions(&self) -> Vector<f32> { self.dimension }
}

impl Render<()> for Wall {
	fn render(&self, renderer : &Renderer,_: &mut ()) {
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions(),
		)
	}
}

impl Interact for Wall {}
