extern crate "2dgl" as tdgl;

use tdgl::game::gameloop::EventHandler;

use event;

pub struct DummyHandler;

impl EventHandler<event::Event> for DummyHandler {
	fn event(&mut self, e: event::Event){
		match e {
			event::Jump => {
				println!("I said jump!")
			}
			_ => {}
		}
	}
}
