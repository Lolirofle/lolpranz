#[deriving(Send,Clone)]
pub enum PlayerEvent{
	///(multiplier) where -1.0 <= multiplier <= 1.0
	Move(f32),
	Jump,
	Action,
}

#[deriving(Send,Clone)]
pub enum Event{
	///(player id,player event)
	Player(u8,PlayerEvent)
}
