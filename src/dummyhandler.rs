extern crate "2dgl" as tdgl;

use tdgl::game::gameloop::EventHandler;

use event;

pub struct DummyHandler;

impl EventHandler<event::Event> for DummyHandler {
	fn event(&mut self, e: event::Event){
		if let event::Player(player_id,event::Jump) = e{
			println!("I said jump, {}!",player_id)
		}
	}
}
