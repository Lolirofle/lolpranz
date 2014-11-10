extern crate "2dgl"as tdgl;
extern crate libc;

use std::collections::hash_map::HashMap;
use core::mem;
use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::{Update,Render,EventHandler};
use tdgl::game::Game;
use tdgl::graphics::renderer::Renderer;
use glfw;
use std::time::Duration;

use event;
use object::Interact;
use player;
use wall;
use dummyhandler;

pub enum TdpgExit{
	Close,
	Restart,
}

pub struct TdpgGame<'a>{
	should_exit: Option<TdpgExit>,

	object_last_id: u32,
	objects       : HashMap<u32,*mut libc::types::common::c95::c_void>,
	renderables   : HashMap<u32,&'a Render<()> + 'a>,//TODO: Layer/depth/render order using BTreeMap<u8,HashMap<u32,&'a Render<()> + 'a>>,
	updatables    : HashMap<u32,&'a mut Update<(u32,&'a TdpgGame<'a>)> + 'a>,
	event_handlers: HashMap<u32,&'a mut EventHandler<event::Event> + 'a>,
	pub interactables : HashMap<u32,&'a mut Interact + 'a>,

	pub gravity: f32,
	pub max_velocity: f32,
}

impl<'a> TdpgGame<'a>{
	pub fn init() -> TdpgGame<'a>{
		let mut game = TdpgGame{
			should_exit: None,

			object_last_id: 1,
			objects       : HashMap::new(),
			renderables   : HashMap::new(),
			updatables    : HashMap::new(),
			event_handlers: HashMap::new(),
			interactables : HashMap::new(),

			gravity       : 0.2,
			max_velocity  : 8.0,
		};

		unsafe{
			let object_ptr = libc::malloc(mem::size_of::<player::Player>() as libc::size_t);
			let object: &'a mut player::Player = (object_ptr as *mut player::Player).as_mut().unwrap();
			game.objects.insert(game.object_last_id,object_ptr);
			*object = player::Player::new();

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.event_handlers.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let object_ptr = libc::malloc(mem::size_of::<wall::Wall>() as libc::size_t);
			let object: &'a mut wall::Wall = (object_ptr as *mut wall::Wall).as_mut().unwrap();
			game.objects.insert(game.object_last_id,object_ptr);
			*object = wall::Wall::new(
				Vector{x: 50.0 ,y: 240.0},
				Vector{x: 320.0,y: 16.0 }
			);

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
		
			game.object_last_id+=1;
		}

		unsafe{
			let object_ptr = libc::malloc(mem::size_of::<wall::Wall>() as libc::size_t);
			let object: &'a mut wall::Wall = (object_ptr as *mut wall::Wall).as_mut().unwrap();
			game.objects.insert(game.object_last_id,object_ptr);
			*object = wall::Wall::new(
				Vector{x: 80.0 ,y: 200.0},
				Vector{x: 16.0,y: 4.0 }
			);

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
		
			game.object_last_id+=1;
		}

		unsafe{
			let object_ptr = libc::malloc(mem::size_of::<dummyhandler::DummyHandler>() as libc::size_t);
			let object: &'a mut dummyhandler::DummyHandler = (object_ptr as *mut dummyhandler::DummyHandler).as_mut().unwrap();
			game.objects.insert(game.object_last_id,object_ptr);
			*object = dummyhandler::DummyHandler;

			game.event_handlers.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut dummyhandler::DummyHandler>(&object));
		
			game.object_last_id+=1;
		}
		
		return game;
	}
}

impl<'a> Game<glfw::WindowEvent,(),TdpgExit> for TdpgGame<'a>{
	fn should_exit(&self) -> Option<TdpgExit>{
		self.should_exit
	}

	fn target_time_per_frame(&self) -> Duration{
		Duration::nanoseconds(1_000_000_000/60)
	}

	fn init_render(&self,_: &Renderer) -> (){()}
}

impl<'a> Update<()> for TdpgGame<'a>{
	fn update(&mut self,_: (),delta_time: Duration){
		unsafe{//TODO: How to fix efficiently
			let self2 = mem::transmute(&*self);

			for (&id,updatable) in self.updatables.iter_mut(){
				updatable.update((id,self2),delta_time);
			}
		}
	}
}

impl<'a> Render<()> for TdpgGame<'a>{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		renderer.clear();

		for (_,renderable) in self.renderables.iter(){
			renderable.render(renderer,&mut ());
		}
	}
}

impl<'a> EventHandler<glfw::WindowEvent> for TdpgGame<'a>{
	fn event(&mut self,event: glfw::WindowEvent){
		match match event{
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
				self.should_exit = Some(TdpgExit::Close);
				None
			},
			glfw::KeyEvent(glfw::KeyR,_,glfw::Press,_) => {
				self.should_exit = Some(TdpgExit::Restart);
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(event::Jump),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(event::Move(-1.0)),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(event::Move( 1.0)),
			
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::Move(0.0)),
			_ => None
		}{
			Some(e) => {
				for (_,handler) in self.event_handlers.iter_mut(){
					handler.event(e);
				}
			},
			None    => {}
		};
	}
}

#[unsafe_destructor]
impl<'a> Drop for TdpgGame<'a>{
	fn drop(&mut self){
		for (_,&object) in self.objects.iter_mut(){unsafe{
			libc::funcs::c95::stdlib::free(mem::transmute(object));
		}}
	}
}
