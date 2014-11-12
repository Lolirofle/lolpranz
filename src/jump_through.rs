extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::Render;
use tdgl::graphics::renderer::Renderer;

use object::{Position,Dimension,Interact};

pub struct JumpThrough {
	pub pos: Vector<f32>,
	pub dim: Vector<f32>,
}

impl Position for JumpThrough {
	fn get_position(&self) -> Vector<f32> { self.pos }
}

impl Dimension for JumpThrough {
	fn get_dimensions(&self) -> Vector<f32> { self.dim }
}

impl Render<()> for JumpThrough {
	fn render(&self, renderer : &Renderer,_: &mut ()) {
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions(),
		)
	}
}

impl Interact for JumpThrough{
	fn is_solid(&self,other: &Interact) -> bool{
		self.pos.y > other.get_position().y + other.get_dimensions().y//TODO: Check for velocity too?
	}
}
