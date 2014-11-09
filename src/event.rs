extern crate "2dgl"as tdgl;

pub enum Event{
	Move(f32),//0.0 <= modifier <= 1.0
	Jump,
	Action,
}
