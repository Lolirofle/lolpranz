use alloc;
use core::mem;
use glfw;
use std::collections::hash_map::HashMap;
use std::ptr;
use std::time::Duration;
use tdgl::data::two_dim::vector;
use tdgl::game::Game;
use tdgl::game::gameloop::{Update,Render,EventHandler};
use tdgl::graphics::renderer::Renderer;

use dummyhandler;
use event;
use item;
use jump_through;
use object::Interact;
use player;
use wall;

pub enum TdpgExit{
	Close,
	Restart,
}

pub struct TdpgGame<'a>{
	should_exit: Option<TdpgExit>,

	object_last_id: u32,
	objects       : HashMap<u32,(*mut u8,uint,uint)>,
	renderables   : HashMap<u32,&'a Render<()> + 'a>,//TODO: Layer/depth/render order using BTreeMap<u8,HashMap<u32,&'a Render<()> + 'a>>,
	updatables    : HashMap<u32,&'a mut Update<(u32,&'a TdpgGame<'a>)> + 'a>,
	event_handlers: HashMap<u32,Sender<event::Game>>,
	pub interactables : HashMap<u32,&'a mut Interact + 'a>,
	//to_be_destroyed: Vec<u32>,

	pub gravity: f32,
	pub max_velocity: f32,
}

impl<'a> TdpgGame<'a>{
	/*pub fn destroy_object(&mut self,id: u32){
		self.to_be_destroyed.push(id);
	}*/

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

		//TODO: Look into core::cell::UnsafeCell (replace some of the code?)
		//TODO: Look into ref counting (alloc::rc) and weak refs for destroying objects (cleaning from the lists)
		//TODO: Create some kind of "create_object" function that generalizes these steps as much as possible
		//TODO: Use channels to signal removement for all the lists?
		//TODO: Split object data for more safety (general data (pos, vel), render data (cache), physics data), but still not entirely safe
		unsafe{
			let (size,align) = (mem::size_of::<player::Player>(),mem::align_of::<player::Player>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut player::Player;
			let object = object_ptr.as_mut().unwrap();
			match player::Player::new(0,vector::Coord{x: 60.0,y: 0.0}){
				(o,transmitter) => {
					ptr::write(object_ptr,o);
					game.event_handlers.insert(game.object_last_id,transmitter);
				}
			};

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<player::Player>(),mem::align_of::<player::Player>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut player::Player;
			let object = object_ptr.as_mut().unwrap();
			match player::Player::new(1,vector::Coord{x: 100.0,y: 0.0}){
				(o,transmitter) => {
					ptr::write(object_ptr,o);
					game.event_handlers.insert(game.object_last_id,transmitter);
				}
			};

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<dummyhandler::DummyHandler>(),mem::align_of::<dummyhandler::DummyHandler>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut dummyhandler::DummyHandler;
			let object = object_ptr.as_mut().unwrap();
			match dummyhandler::DummyHandler::new(){
				(o,transmitter) => {
					ptr::write(object_ptr,o);
					game.event_handlers.insert(game.object_last_id,transmitter);
				}
			};

			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut dummyhandler::DummyHandler>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<wall::Wall>(),mem::align_of::<wall::Wall>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut wall::Wall;
			let object = object_ptr.as_mut().unwrap();
			ptr::write(object_ptr,wall::Wall{
				pos: vector::Coord{x: 50.0 ,y: 240.0},
				dim: vector::Coord{x: 320.0,y: 16.0 }
			});

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<wall::Wall>(),mem::align_of::<wall::Wall>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut wall::Wall;
			let object = object_ptr.as_mut().unwrap();
			ptr::write(object_ptr,wall::Wall{
				pos: vector::Coord{x: 80.0 ,y: 200.0},
				dim: vector::Coord{x: 16.0,y: 4.0 }
			});

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<jump_through::JumpThrough>(),mem::align_of::<jump_through::JumpThrough>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut jump_through::JumpThrough;
			let object = object_ptr.as_mut().unwrap();
			ptr::write(object_ptr,jump_through::JumpThrough{
				pos: vector::Coord{x: 112.0 ,y: 200.0},
				dim: vector::Coord{x: 16.0,y: 4.0 }
			});

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut jump_through::JumpThrough>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut jump_through::JumpThrough>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<item::Item>(),mem::align_of::<item::Item>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object_ptr = object_ptr as *mut item::Item;
			let object = object_ptr.as_mut().unwrap();
			ptr::write(object_ptr,item::Item{
				pos: vector::Coord{x: 160.0 ,y: 220.0},
				dim: vector::Coord{x: 8.0,y: 8.0 }
			});

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut item::Item>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut item::Item>(&object));

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
			glfw::KeyEvent(glfw::Key::Escape,_,glfw::Press,_) |
			glfw::CloseEvent => {
				self.should_exit = Some(TdpgExit::Close);
				None
			},
			glfw::KeyEvent(glfw::Key::R,_,glfw::Press,_) => {
				self.should_exit = Some(TdpgExit::Restart);
				None
			},
			glfw::KeyEvent(glfw::Key::Space,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::Key::Up   ,_,glfw::Press,_)  => Some(event::Game::Player(0,event::Player::Jump)),
			glfw::KeyEvent(glfw::Key::Left ,_,glfw::Press,_)  => Some(event::Game::Player(0,event::Player::Move(-1.0))),
			glfw::KeyEvent(glfw::Key::Right,_,glfw::Press,_)  => Some(event::Game::Player(0,event::Player::Move(1.0))),
			glfw::KeyEvent(glfw::Key::Left ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::Key::Right,_,glfw::Release,_) => Some(event::Game::Player(0,event::Player::Move(0.0))),

			glfw::KeyEvent(glfw::Key::W   ,_,glfw::Press,_)  => Some(event::Game::Player(1,event::Player::Jump)),
			glfw::KeyEvent(glfw::Key::A ,_,glfw::Press,_)  => Some(event::Game::Player(1,event::Player::Move(-1.0))),
			glfw::KeyEvent(glfw::Key::D,_,glfw::Press,_)  => Some(event::Game::Player(1,event::Player::Move(1.0))),
			glfw::KeyEvent(glfw::Key::A ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::Key::D,_,glfw::Release,_) => Some(event::Game::Player(1,event::Player::Move(0.0))),
			_ => None
		}{
			Some(e) => {
				for transmitter in self.event_handlers.values(){
					transmitter.send(e);
				}
			},
			None    => {}
		};
	}
}

#[unsafe_destructor]
impl<'a> Drop for TdpgGame<'a>{
	fn drop(&mut self){
		for &(object,size,align) in self.objects.values(){unsafe{
			drop(*object);//TODO: Should we drop? (destructor)
			alloc::heap::deallocate(mem::transmute(object),size,align);
		}}
	}
}
