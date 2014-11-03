extern crate "2dgl"as tdgl;

use tdgl::data::vector::Vector2;

pub trait Position {
	fn get_position(&self) -> Vector2<f32>;
}

//any object that has a box
pub trait Collision {
	fn get_dimensions(&self) -> Vector2<f32>;
}

pub trait Interactable : Collision+Position {
	fn has_point(&self, v: Vector2<f32>) -> bool {
		let Vector2(point_x,point_y) = v;
		let Vector2(x1,y1) = self.get_position();
		let Vector2(x2,y2) = self.get_position() + self.get_dimensions();
		x1 <= point_x && point_x <= x2 && y1 <= point_y && point_y <= y2
	}

	fn collides_with<T:Interactable>(&self, other : T, delta_position : Vector2<f32>) -> bool {
		let Vector2(self_x1, self_y1) = self.get_position();
		let Vector2(self_x2, self_y2) = self.get_position() + self.get_dimensions();
		let Vector2(other_x1, other_y1) = other.get_position();
		let Vector2(other_x2, other_y2) = other.get_position() + other.get_dimensions() + delta_position;
		self_x1 <= other_x2 && self_x2 >= other_x1 &&
			self_y1 <= other_y2 && self_y2 >= other_y1
	}
}
