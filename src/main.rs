#![feature(unsafe_destructor)]

extern crate "2dgl" as tdgl;
extern crate alloc;
extern crate collections;
extern crate core;
extern crate glfw;
extern crate num;

use game::{TdpgGame,TdpgExit};
use tdgl::game_handler::GameHandler as GameHandlerTrait;
use tdgl::game_handler::singlethreaded::GameHandler;
use tdgl::graphics::direct_mode::renderer::Renderer;

mod dummyhandler;
mod event;
mod game;
mod glfw_game;
mod item;
mod jump_through;
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
