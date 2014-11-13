extern crate "2dgl"as tdgl;
extern crate alloc;

use std::collections::hash_map::HashMap;
use core::mem;
use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop::{Update,Render,EventHandler};
use tdgl::game::Game;
use tdgl::graphics::renderer::Renderer;
use glfw;
use std::time::Duration;

use event;
use item;
use jump_through;
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
	objects       : HashMap<u32,(*mut u8,uint,uint)>,
	renderables   : HashMap<u32,&'a Render<()> + 'a>,//TODO: Layer/depth/render order using BTreeMap<u8,HashMap<u32,&'a Render<()> + 'a>>,
	updatables    : HashMap<u32,&'a mut Update<(u32,&'a TdpgGame<'a>)> + 'a>,
	event_handlers: HashMap<u32,Sender<event::Event>>,
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

		//TODO: Look into std::cell::UnsafeCell (replace some of the code?)
		unsafe{
			let (size,align) = (mem::size_of::<player::Player>(),mem::align_of::<player::Player>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut player::Player).as_mut().unwrap();
			let (o,transmitter) = player::Player::new(0,Vector{x: 60.0,y: 0.0});
			*object = o;
			game.event_handlers.insert(game.object_last_id,transmitter);

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<player::Player>(),mem::align_of::<player::Player>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut player::Player).as_mut().unwrap();
			let (o,transmitter) = player::Player::new(1,Vector{x: 100.0,y: 0.0});
			*object = o;
			game.event_handlers.insert(game.object_last_id,transmitter);

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut player::Player>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<dummyhandler::DummyHandler>(),mem::align_of::<dummyhandler::DummyHandler>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut dummyhandler::DummyHandler).as_mut().unwrap();
			let (o,transmitter) = dummyhandler::DummyHandler::new();
			*object = o;
			game.event_handlers.insert(game.object_last_id,transmitter);
		
			game.updatables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut dummyhandler::DummyHandler>(&object));

			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<wall::Wall>(),mem::align_of::<wall::Wall>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut wall::Wall).as_mut().unwrap();
			*object = wall::Wall{
				pos: Vector{x: 50.0 ,y: 240.0},
				dim: Vector{x: 320.0,y: 16.0 }
			};

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
		
			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<wall::Wall>(),mem::align_of::<wall::Wall>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut wall::Wall).as_mut().unwrap();
			*object = wall::Wall{
				pos: Vector{x: 80.0 ,y: 200.0},
				dim: Vector{x: 16.0,y: 4.0 }
			};

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut wall::Wall>(&object));
		
			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<jump_through::JumpThrough>(),mem::align_of::<jump_through::JumpThrough>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut jump_through::JumpThrough).as_mut().unwrap();
			*object = jump_through::JumpThrough{
				pos: Vector{x: 112.0 ,y: 200.0},
				dim: Vector{x: 16.0,y: 4.0 }
			};

			game.renderables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut jump_through::JumpThrough>(&object));
			game.interactables.insert(game.object_last_id,mem::transmute_copy::<_,&'a mut jump_through::JumpThrough>(&object));
		
			game.object_last_id+=1;
		}

		unsafe{
			let (size,align) = (mem::size_of::<item::Item>(),mem::align_of::<item::Item>());
			let object_ptr = alloc::heap::allocate(size,align);
			game.objects.insert(game.object_last_id,(object_ptr,size,align));

			let object = (object_ptr as *mut item::Item).as_mut().unwrap();
			*object = item::Item{
				pos: Vector{x: 160.0 ,y: 220.0},
				dim: Vector{x: 8.0,y: 8.0 }
			};

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
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) |
			glfw::CloseEvent => {
				self.should_exit = Some(TdpgExit::Close);
				None
			},
			glfw::KeyEvent(glfw::KeyR,_,glfw::Press,_) => {
				self.should_exit = Some(TdpgExit::Restart);
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(event::Player(0,event::Jump)),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(event::Player(0,event::Move(-1.0))),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(event::Player(0,event::Move(1.0))),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::Player(0,event::Move(0.0))),

			glfw::KeyEvent(glfw::KeyW   ,_,glfw::Press,_)  => Some(event::Player(1,event::Jump)),
			glfw::KeyEvent(glfw::KeyA ,_,glfw::Press,_)  => Some(event::Player(1,event::Move(-1.0))),
			glfw::KeyEvent(glfw::KeyD,_,glfw::Press,_)  => Some(event::Player(1,event::Move(1.0))),
			glfw::KeyEvent(glfw::KeyA ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyD,_,glfw::Release,_) => Some(event::Player(1,event::Move(0.0))),
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
			alloc::heap::deallocate(mem::transmute(object),size,align);
		}}
	}
}
