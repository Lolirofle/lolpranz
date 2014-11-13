use tdgl::data::vector2::coord_vector::Vector;

pub trait Position {
	fn get_position(&self) -> Vector<f32>;
}

pub trait Velocity{
	fn get_velocity(&self) -> Vector<f32>;
}

/// Object that has a dimension and therefore also a collision box
pub trait Dimension {
	fn get_dimensions(&self) -> Vector<f32>;
}

pub trait Interact: Dimension + Position{
	fn is_solid(&self,other: &Interact) -> bool;

	fn has_point(&self, v: Vector<f32>) -> bool {
		let min = self.get_position();
		let max = min + self.get_dimensions();
		
		min.x <= v.x &&
		max.x >= v.x &&
		min.y <= v.y &&
		max.y >= v.y
	}

	fn collides_with(&self, other : &Interact, delta_position : Vector<f32>) -> bool {
		let self_min  = self.get_position();
		let self_max  = self_min + self.get_dimensions();
		let other_min = other.get_position() + delta_position;//TODO: Delta position here?
		let other_max = other_min + other.get_dimensions();
		
		self_min.x <= other_max.x &&
		self_max.x >= other_min.x &&
		self_min.y <= other_max.y &&
		self_max.y >= other_min.y
	}

	/// Implementation is based on SAT algorithm.
	/// Current code only works for axis-aligned rectangles
	fn collision_check(&self,other: &Interact) -> Option<Vector<f32>>{//TODO: REname?
		let self_center  = self.get_dimensions()/2.0;
		let other_center = other.get_dimensions()/2.0;
		let gap = (self_center + other_center) - ((self.get_position() + self_center) - (other.get_position() + other_center)).abs();//TODO: Able to simplify this expression by mathematical logic?

		return if gap.x>=0.0 && gap.y>=0.0{
			Some(gap)
		}else{
			None
		}
	}
}

pub trait Destroyed{//TODO: Destroy based on object id. When an object reports that it is destroyed, the game should take care of the garbage by filtering the object in question from the lists
	fn destroyed() -> bool;
}
