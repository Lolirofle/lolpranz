use game::TdpgGame;
use std::time::Duration;
use tdgl::game::gameloop::Update;

use event;

pub struct DummyHandler{
	event_receiver: Receiver<event::Game>
}

impl DummyHandler{
	pub fn new() -> (DummyHandler,Sender<event::Game>){
		let (transmitter,receiver) = channel();
		return (DummyHandler{
			event_receiver: receiver,
		},transmitter);
	}
}

impl<'a> Update<(u32,&'a TdpgGame<'a>)> for DummyHandler{
	fn update(&mut self,_: (u32,&TdpgGame),_ : Duration){
		while let Ok(e) = self.event_receiver.try_recv(){
			match e{
				event::Game::Player(player_id,event::Player::Jump) => println!("I said jump, {}!",player_id),
				_ => {}
			}
		}
	}
}
