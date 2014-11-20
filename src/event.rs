#[deriving(Send,Clone)]
pub enum Player{
	///(multiplier) where -1.0 <= multiplier <= 1.0
	Move(f32),
	Jump,
	Action,
}

#[deriving(Send,Clone)]
pub enum Game{
	///(player id,player event)
	Player(u8,Player)
}
