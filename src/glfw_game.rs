use glfw::{Context,Window};
use glfw;
use std::time::Duration;
use tdgl::game::Game;
use tdgl::game::gameloop;
use tdgl::graphics::renderer::Renderer;

pub struct GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	glfw: glfw::Glfw,
	pub window: (glfw::Window,Receiver<(f64,glfw::WindowEvent)>),
	game: G,
}

impl<'g,Exit,G> GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	pub fn using_game(game: G) -> GlfwGame<'g,Exit,G>{
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		let window = glfw.create_window(640,480,"Lolpranz",glfw::Windowed).expect("Failed to create GLFW window.");

		GlfwGame{
			glfw: glfw,
			window: window,
			game: game,
		}
	}

	pub fn use_game(&mut self,game: G){
		self.game = game;
	}
}

impl<'g,Exit,G> Game<(),(),Exit> for GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	fn should_exit(&self) -> Option<Exit>{
		if let Some(exit) = self.game.should_exit(){
			self.window.0.set_should_close(true);
			return Some(exit);
		}
		return None;
	}

	fn target_time_per_frame(&self) -> Duration{
		self.game.target_time_per_frame()
	}

	fn init_render(&self,renderer: &Renderer) -> (){
		self.glfw.make_context_current(Some(&self.window.0));
		
		//Window
		//self.glfw.window_hint(glfw::ContextVersion(3,2));
		//self.glfw.window_hint(glfw::OpenglForwardCompat(true));
		//self.glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

		//Initialize window
		self.window.0.set_all_polling(true);
		self.window.0.make_current();
		self.glfw.set_swap_interval(0);
		
		renderer.init_projection(0,0,640,480);

		return ();
	}
}

impl<'g,Exit,G> gameloop::Update<()> for GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	fn update(&mut self,_: (),delta_time: Duration){
		self.game.update((),delta_time);
	}
}

impl<'g,Exit,G> gameloop::Render<()> for GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	fn render(&self,renderer: &Renderer,_: &mut ()){
		self.game.render(renderer,&mut ());
		self.window.0.swap_buffers();
	}
}

impl<'g,Exit,G> gameloop::EventHandler<()> for GlfwGame<'g,Exit,G>
	where G: Game<glfw::WindowEvent,(),Exit> + 'g
{
	fn event(&mut self,_: ()){
		self.glfw.poll_events();

		for (_,e) in glfw::flush_messages(&self.window.1){
			self.game.event(e);
		}
	}
}
