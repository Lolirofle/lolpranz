extern crate "2dgl"as tdgl;

use tdgl::data::vector2::coord_vector::Vector;
use tdgl::game::gameloop;
use tdgl::game::Game;
use tdgl::graphics::renderer::Renderer;
use glfw;
use std::mem;
use std::time::Duration;

use event;
use player;
use wall;

pub struct TdpgGame<'a>{
	should_exit: bool,

	player        : player::Player,
	wall          : wall::Wall,
	renderables   : Vec<&'a mut gameloop::Render<()> + 'a>,
	updaters      : Vec<&'a mut gameloop::Update<TdpgGame<'a>> + 'a>,
	event_handlers: Vec<&'a mut gameloop::EventHandler<event::Event> + 'a>,
}

impl<'a> TdpgGame<'a>{
	pub fn init() -> TdpgGame<'a>{
		return TdpgGame{
			should_exit: false,

			player: player::Player::new(),
			wall  : wall::Wall::new(Vector{x: 50.0,y: 240.0},Vector{x: 16f32,y: 16f32}),
			
			renderables   : Vec::with_capacity(20u),
			updaters      : Vec::with_capacity(20u),
			event_handlers: Vec::with_capacity(20u),
		};
	}
}

impl<'a> Game<glfw::WindowEvent,()> for TdpgGame<'a>{
	fn should_exit(&self) -> bool{
		self.should_exit
	}

	fn target_time_per_frame(&self) -> Duration{
		Duration::nanoseconds(1_000_000_000/60)
	}

	fn init_render(&self,renderer: &Renderer) -> (){()}
}

impl<'a> gameloop::Update<()> for TdpgGame<'a>{
	fn update(&mut self,_: (),delta_time: Duration){
		unsafe{//TODO: How to fix efficiently
			let self2 = mem::transmute(&*self);
			self.player.update(self2,delta_time);
		}
	}
}

impl<'a> gameloop::Render<()> for TdpgGame<'a>{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		renderer.clear();
		self.player.render(renderer,&mut ());
	}
}

impl<'a> gameloop::EventHandler<glfw::WindowEvent> for TdpgGame<'a>{
	fn event(&mut self,event: glfw::WindowEvent){
		match match event{
			glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
				self.should_exit = true;
				None
			},
			glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
			glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(event::Jump),
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(event::Move(Vector{x: -1.0,y: 0.0})),
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(event::Move(Vector{x:  1.0,y: 0.0})),
			
			glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
			glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::StopMove),
			_ => None
		}{
			Some(e) => {self.player.event(e);},
			None    => {}
		};
	}
}
