extern crate "2dgl"as tdgl;

use tdgl::data::vector::Vector2;

pub enum Event{
	Move(Vector2<f32>),
	StopMove,
	Jump,
	Action,
}
