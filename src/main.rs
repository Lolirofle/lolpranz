#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate glfw;

use lolirofle::tdpg::TdpgGame;
use lolirofle::game_handler::GameHandler as GameHandlerTrait;
use lolirofle::game_handler::singlethreaded::GameHandler;
mod lolirofle;

fn main(){
	GameHandler::<TdpgGame>.run();
}
