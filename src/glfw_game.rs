extern crate "2dgl"as tdgl;

use tdgl::game::gameloop;
use tdgl::game::Game;
use tdgl::graphics::renderer::Renderer;
//use tdgl::gl;
use glfw;
use glfw::{Context,Window};
use std::time::Duration;

pub struct GlfwGame<'g>{
	glfw: glfw::Glfw,
	pub window: (glfw::Window,Receiver<(f64,glfw::WindowEvent)>),
	game: &'g mut Game<glfw::WindowEvent,()>+'g,
	render_context: glfw::RenderContext,
}

impl<'g> GlfwGame<'g>{
	pub fn using_game<G: Game<glfw::WindowEvent,()>>(game: &'g mut G) -> GlfwGame<'g>{
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		let mut window = glfw.create_window(640,480,"GLTest",glfw::Windowed).expect("Failed to create GLFW window.");
		let render_context = window.0.render_context();

		GlfwGame{
			glfw: glfw,
			window: window,
			game: game,
			render_context: render_context,
		}
	}
}

impl<'g> Game<(),()> for GlfwGame<'g>{
	fn should_exit(&self) -> bool{
		self.game.should_exit() || self.window.0.should_close()
	}

	fn target_time_per_frame(&self) -> Duration{
		self.game.target_time_per_frame()
	}

	fn init_render(&self,renderer: &Renderer) -> (){
		self.render_context.make_current();
		
		//Window
		/*glfw.window_hint(glfw::ContextVersion(3,2));
		glfw.window_hint(glfw::OpenglForwardCompat(true));
		glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));*/

		//Initialize window
		//self.window.0.set_all_polling(true);
		self.window.0.set_key_polling(true);
		self.window.0.make_current();
		self.glfw.set_swap_interval(0);
		
		renderer.init_projection(0,0,640,480);

		return ();
	}
}

impl<'g> gameloop::Update<()> for GlfwGame<'g>{
	fn update(&mut self,_: (),delta_time: Duration){
		self.game.update((),delta_time);
	}
}

impl<'g> gameloop::Render<()> for GlfwGame<'g>{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		self.game.render(renderer,&mut ());
		self.render_context.swap_buffers();
	}
}

impl<'g> gameloop::EventHandler<()> for GlfwGame<'g>{
	fn event(&mut self,_: ()){
		self.glfw.poll_events();

		for (_,e) in glfw::flush_messages(&self.window.1){
			self.game.event(e);
		}
	}
}
