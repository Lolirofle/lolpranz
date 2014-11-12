extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::Render;
use tdgl::graphics::renderer::Renderer;

use object::{Position,Dimension,Interact};

pub struct Item{
	pub pos: Vector<f32>,
	pub dim: Vector<f32>,
}

impl Position for Item {
	fn get_position(&self) -> Vector<f32> { self.pos }
}

impl Dimension for Item {
	fn get_dimensions(&self) -> Vector<f32> { self.dim }
}

impl Render<()> for Item {
	fn render(&self, renderer : &Renderer,_: &mut ()) {
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions(),
		)
	}
}

impl Interact for Item{
	fn is_solid(&self,other: &Interact) -> bool{false}
}
