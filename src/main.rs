#![feature(globs)]
#![feature(tuple_indexing)]
#![feature(unsafe_destructor)]
#![feature(if_let)]

extern crate core;
extern crate collections;
extern crate glfw;
extern crate "2dgl"as tdgl;

use game::{TdpgGame,TdpgExit};
use tdgl::game_handler::GameHandler as GameHandlerTrait;
use tdgl::game_handler::singlethreaded::GameHandler;
use tdgl::graphics::direct_mode::renderer::Renderer;

mod dummyhandler;
mod event;
mod game;
mod glfw_game;
mod object;
mod player;
mod wall;

fn main(){
	let game_handler = GameHandler;
	let mut game = glfw_game::GlfwGame::using_game(TdpgGame::init());

	loop{
		match game_handler.run(Renderer::new(|s| game.window.0.get_proc_address(s)),&mut game){
			TdpgExit::Close   => break,
			TdpgExit::Restart => {
				game.use_game(TdpgGame::init());
			},
		};
	}
}
