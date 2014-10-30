use lolirofle::data::vector::Vector2;
use lolirofle::gl::renderer::Renderer;
use lolirofle::game::gameloop::Renderable;

pub trait Position {
    fn get_position(&self) -> Vector2<f32>;
}

//any object that has a box
pub trait Collision {
    fn get_dimensions(&self) -> Vector2<f32>;
}

pub trait Interactable : Collision+Position {
    fn has_point(&self, v: Vector2<f32>) -> bool {
        let Vector2(pointX,pointY)   = v;
        let Vector2(x1,y1) = self.get_position();
        let Vector2(x2,y2) = self.get_position() + self.get_dimensions();
        x1 <= pointX && pointX <= x2 && y1 <= pointY && pointY <= y2
    }

    fn collides_with<T:Interactable>(&self, other : T, delta_position : Vector2<f32>) -> bool {
        let Vector2(self_x1, self_y1) = self.get_position();
        let Vector2(self_x2, self_y2) = self.get_position() + self.get_dimensions();
        let Vector2(other_x1, other_y1) = other.get_position();
        let Vector2(other_x2, other_y2) = other.get_position() + other.get_dimensions() + delta_position;
        self_x1 <= other_x2 && self_x2 >= other_x1 &&
            self_y1 <= other_y2 && self_y2 >= other_y1
    }
}

pub struct Wall {
    pos       : Vector2<f32>,
    dimension : Vector2<f32>,
}

impl Wall {
    pub fn new(p : Vector2<f32>, d : Vector2<f32>) -> Wall {
        Wall {pos : p, dimension : d}
    }
}

impl Position for Wall {
    fn get_position(&self) -> Vector2<f32> { self.pos }
}

impl Collision for Wall {
    fn get_dimensions(&self) -> Vector2<f32> { self.dimension }
}

impl Renderable for Wall {
    fn render(&self, renderer : &Renderer) {
        renderer.render_rectangle(
            self.get_position(),
            self.get_dimensions(),
            )
    }
}

impl Interactable for Wall {}
