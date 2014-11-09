extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;

pub trait Position {
	fn get_position(&self) -> Vector<f32>;
}

//any object that has a box
pub trait Collision {
	fn get_dimensions(&self) -> Vector<f32>;
}

pub trait Interactable : Collision+Position {
	fn has_point(&self, v: Vector<f32>) -> bool {
		let min = self.get_position();
		let max = min + self.get_dimensions();
		min.x <= v.x && v.x <= max.x && min.y <= v.y && v.y <= max.y
	}

	fn collides_with<T:Interactable>(&self, other : T, delta_position : Vector<f32>) -> bool {
		let self_min  = self.get_position();
		let self_max  = self_min + self.get_dimensions();
		let other_min = other.get_position();
		let other_max = other_min + other.get_dimensions() + delta_position;
		
		self_min.x <= other_max.x && self_max.x >= other_min.x &&
		self_min.y <= other_max.y && self_max.y >= other_min.y
	}
}
