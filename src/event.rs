extern crate "2dgl"as tdgl;

pub enum PlayerEvent{
	///(multiplier) where -1.0 <= multiplier <= 1.0
	Move(f32),
	Jump,
	Action,
}

pub enum Event{
	///(player id,player event)
	Player(u8,PlayerEvent)
}
