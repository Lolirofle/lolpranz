extern crate gl;
use gl::types::*;
use std::mem;
use std::ptr;
use lolirofle::vector::Vector2;
use lolirofle::gl::*;
use lolirofle::gl::vertex_object::VertexObject;

//Import shader sources
const VERTEX_SHADER_SRC:   &'static str = include_str!("vertex_shader.glsl");
const FRAGMENT_SHADER_SRC: &'static str = include_str!("fragment_shader.glsl");

//Vertex coordinates for the triangles making up a square
const VERTICES_SQUARE: [GLfloat, ..12] = [
	0.0,0.0,
	0.0,1.0,
	1.0,1.0,

	1.0,1.0,
	1.0,0.0,
	0.0,0.0,
];

fn create_unit_square() -> VertexObject{
	let size = (VERTICES_SQUARE.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

	//Initialize Vertex Buffer Object, copying the vertex data to it
	let mut vbo = 0;unsafe{
		gl::GenBuffers(1,&mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			size,
			mem::transmute(&VERTICES_SQUARE[0]),
			gl::STATIC_DRAW
		);
	}

	//Initialize Vertex Array Object
	let mut vao = 0;unsafe{
		gl::GenVertexArrays(1,&mut vao);
	}
	
	return VertexObject{
		array: vao,
		buffer: vbo,
		size: size
	};
}

pub struct Renderer{
	unit_square: VertexObject,

	position_loc: GLint,
	size_loc: GLint,
	framebuffer_size_loc: GLint,

	vertex_shader: GLuint,
	fragment_shader: GLuint,
	shader_program: GLuint,
}
impl Renderer{
	pub fn initiated() -> Renderer{
		let vertex_shader   = shaders::compile_shader(VERTEX_SHADER_SRC  ,gl::VERTEX_SHADER);
		let fragment_shader = shaders::compile_shader(FRAGMENT_SHADER_SRC,gl::FRAGMENT_SHADER);
		let shader_program  = shaders::link_program(vertex_shader,fragment_shader);

		//Use shader program
		gl::UseProgram(shader_program);

		unsafe{
			///////////////////////////////////////////////
			// Prepare uniform variable locations

			let position_loc         = "pos".with_c_str(|ptr| gl::GetUniformLocation(shader_program,ptr));
			let size_loc             = "size".with_c_str(|ptr| gl::GetUniformLocation(shader_program,ptr));
			let framebuffer_size_loc = "frameBufferSize".with_c_str(|ptr| gl::GetUniformLocation(shader_program,ptr));

			///////////////////////////////////////////////
			// Prepare attribute (in) variable locations

			let vertex_coord = "vertexCoord".with_c_str(|ptr| gl::GetAttribLocation(shader_program,ptr));

			///////////////////////////////////////////////
			// Prepare unit square
			let unit_square = create_unit_square();
			gl::BindVertexArray(unit_square.array);

			gl::EnableVertexAttribArray(vertex_coord as GLuint);
			gl::VertexAttribPointer(vertex_coord as GLuint,2,gl::FLOAT,gl::FALSE as GLboolean,0,ptr::null());

			//Fragment shader data
			"out_color".with_c_str(|ptr| gl::BindFragDataLocation(shader_program,0,ptr));

			return Renderer{
				unit_square: unit_square,
				position_loc: position_loc,
				framebuffer_size_loc: framebuffer_size_loc,
				size_loc: size_loc,
				vertex_shader: vertex_shader,
				fragment_shader: fragment_shader,
				shader_program: shader_program,
			}
		}
	}

	pub fn render_rectangle(&self,Vector2(x,y): Vector2<GLfloat>,Vector2(w,h): Vector2<GLfloat>){
		gl::Uniform2f(self.position_loc,x,y);
		gl::Uniform2f(self.size_loc    ,w,h);

		gl::DrawArrays(gl::TRIANGLES,0,self.unit_square.size as GLint);
	}

	pub fn init_projection(&self,x:GLint,y:GLint,width:GLuint,height:GLuint){
		gl::Viewport(x,y,width as GLint,height as GLint);
		gl::Uniform2f(self.framebuffer_size_loc,width as GLfloat,height as GLfloat);
	}
}
impl Drop for Renderer{
	fn drop(&mut self){
		//Free
		gl::DeleteProgram(self.shader_program);
		gl::DeleteShader(self.fragment_shader);
		gl::DeleteShader(self.vertex_shader);
		unsafe{
			gl::DeleteBuffers(1,&self.unit_square.buffer);
			gl::DeleteVertexArrays(1,&self.unit_square.array);
		}
	}
}
