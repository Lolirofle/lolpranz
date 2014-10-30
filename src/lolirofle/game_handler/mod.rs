use lolirofle::game::Game;

pub mod singlethreaded;

pub trait GameHandler<G>
	where G: Game{
	fn run(&self);
}
