#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate glfw;
extern crate "2dgl"as tdgl;

use game::TdpgGame;
use tdgl::game_handler::GameHandler as GameHandlerTrait;
use tdgl::game_handler::singlethreaded::GameHandler;
use tdgl::graphics::direct_mode::renderer::Renderer;

mod event;
mod game;
mod glfw_game;
mod object;
mod player;
mod wall;

fn main(){
	let mut game1 = TdpgGame::init();
	let mut game2 = glfw_game::GlfwGame::using_game(&mut game1);
	let game_handler: GameHandler<glfw_game::GlfwGame,Renderer,()> = GameHandler;
	game_handler.run(Renderer::new(|s| game2.window.0.get_proc_address(s)),&mut game2);
}
