use game::TdpgGame;
use std::time::Duration;
use tdgl::game::gameloop::Update;

use event;

pub struct DummyHandler{
	event_receiver: Receiver<event::Event>
}

impl DummyHandler{
	pub fn new() -> (DummyHandler,Sender<event::Event>){
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
				event::Player(player_id,event::Jump) => println!("I said jump, {}!",player_id),
				_ => {}
			}
		}
	}
}
