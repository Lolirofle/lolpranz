extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;

pub enum Event{
	Move(Vector<f32>),
	StopMove,
	Jump,
	Action,
}
