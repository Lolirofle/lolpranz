extern crate "2dgl" as tdgl;

use tdgl::lolirofle::game::gameloop::*;
use lolirofle::tdpg::event;
use lolirofle::tdpg::TdpgGame;

pub struct DummyHandler {
    unit : (),
}

impl DummyHandler {
    pub fn new() -> DummyHandler {
        DummyHandler {unit : ()}
    }
}

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
