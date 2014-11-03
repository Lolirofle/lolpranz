#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate glfw;
extern crate "2dgl"as tdgl;

use tdgl::game_handler::singlethreaded::GameHandler;
use tdgl::graphics::direct_mode::renderer::Renderer;

use game::TdpgGame;

mod event;
mod game;
mod object;
mod player;
mod wall;

pub struct GlfwInstance{
	glfw: mut Glfw
	window: &mut glfw::Window,
	events: Receiver<(f64,glfw::WindowEvent)>,
}
impl GlfwInstance{
	fn init() -> GlfwInstance{
		GlfwInstance{
			glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
			(mut window,events) = glfw.create_window(640,480,"GLTest",glfw::Windowed).expect("Failed to create GLFW window."),
			//let render_context = window.render_context();
		}
	}

	fn init_render(&self){
		render_context.make_current();
		
		//Window
		/*glfw.window_hint(glfw::ContextVersion(3,2));
		glfw.window_hint(glfw::OpenglForwardCompat(true));
		glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));*/

		//Initialize window
		window.set_all_polling(true);
		window.make_current();
		glfw.set_swap_interval(0);
	
		gl::load_with(|s| window.get_proc_address(s));
	}

	fn render(&self){

		self.render_context.swap_buffers();
	}

	fn event(&self) -> Iterator<event::Event>{
		self.glfw.poll_events();
		self.glfw::flush_messages(&self.events).map(|(_,event)| {
			match event{
				glfw::KeyEvent(glfw::KeyEscape,_,glfw::Press,_) => {
					window.set_should_close(true);
					None
				},
				glfw::KeyEvent(glfw::KeySpace,_,glfw::Press,_) |
				glfw::KeyEvent(glfw::KeyUp   ,_,glfw::Press,_)  => Some(event::Jump),
				glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Press,_)  => Some(event::Move(Vector2::new(-1.0,0.0))),
				glfw::KeyEvent(glfw::KeyRight,_,glfw::Press,_)  => Some(event::Move(Vector2::new( 1.0,0.0))),
				
				glfw::KeyEvent(glfw::KeyLeft ,_,glfw::Release,_) |
				glfw::KeyEvent(glfw::KeyRight,_,glfw::Release,_) => Some(event::StopMove),
				_ => None
			}
		})
	}
}


fn main(){
	let renderer: R = Renderer::initiated();
	renderer.init_projection(0,0,640,480);

	GameHandler::<TdpgGame>.run();
}
