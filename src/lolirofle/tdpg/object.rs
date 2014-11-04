extern crate "2dgl"as tdgl;
extern crate std;

use tdgl::lolirofle::data::vector::Vector2;

pub trait Position{
	fn get_position(&self) -> Vector2<f32>;
}

pub trait Velocity{
	fn get_velocity(&self) -> Vector2<f32>;
}

//any object that has a box
pub trait Collision{
	fn get_dimensions(&self) -> Vector2<f32>;
}

pub trait Interactable: Collision + Position{
	fn has_point(&self, v: Vector2<f32>) -> bool {
		let Vector2(point_x,point_y) = v;
		let Vector2(x1,y1) = self.get_position();
		let Vector2(x2,y2) = self.get_position() + self.get_dimensions();

		x1 <= point_x &&
		x2 >= point_x &&
		y1 <= point_y &&
		y2 >= point_y
	}

	fn collides_with<T: Interactable>(&self, other : T, delta_position : Vector2<f32>) -> bool {
		let Vector2(self_x1, self_y1) = self.get_position();
		let Vector2(self_x2, self_y2) = self.get_position() + self.get_dimensions();
		let Vector2(other_x1, other_y1) = other.get_position() + delta_position;//TODO: Delta position here?
		let Vector2(other_x2, other_y2) = other.get_position() + other.get_dimensions();
		
		self_x1 <= other_x2 &&
		self_x2 >= other_x1 &&
		self_y1 <= other_y2 &&
		self_y2 >= other_y1
	}

	/// Implementation is based on SAT algorithm.
	/// Current code only works for axis-aligned rectangles
	fn collision_check<T: Interactable>(&self,other: T) -> Option<Vector2<f32>>{//TODO: REname?
		let self_center  = self.get_dimensions()/2.0;
		let other_center = other.get_dimensions()/2.0;
		let gap = (self_center + other_center) - ((self.get_position() + self_center) - (other.get_position() + other_center)).abs();//TODO: Able to simplify this expression by mathematical logic?
		let Vector2(gap_x,gap_y) = gap;

		return if gap_x>=0.0 && gap_y>=0.0{
			Some(gap)
		}else{
			None
		}
	}
}
