use tdgl::data::two_dim::vector;
use tdgl::game::gameloop::Render;
use tdgl::graphics::renderer::Renderer;

use object::{Position,Dimension,Interact};

pub struct Wall{
	pub pos: vector::Coord<f32>,
	pub dim: vector::Coord<f32>,
}

impl Position for Wall {
	fn get_position(&self) -> vector::Coord<f32> { self.pos }
}

impl Dimension for Wall {
	fn get_dimensions(&self) -> vector::Coord<f32> { self.dim }
}

impl Render<()> for Wall {
	fn render(&self, renderer : &Renderer,_: &mut ()) {
		renderer.render_rectangle(
			self.get_position(),
			self.get_dimensions(),
		)
	}
}

impl Interact for Wall{
	fn is_solid(&self,_: &Interact) -> bool{true}
}
