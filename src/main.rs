#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate glfw;
extern crate "2dgl"as tdgl;

use lolirofle::tdpg::TdpgGame;
use tdgl::lolirofle::game_handler::GameHandler as GameHandlerTrait;
use tdgl::lolirofle::game_handler::singlethreaded::GameHandler;
use tdgl::lolirofle::gl::direct_mode::renderer::Renderer;
mod lolirofle;

fn main(){
	GameHandler::<TdpgGame,Renderer>.run();
}
