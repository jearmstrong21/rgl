use crate::{VertexArray, buffer, Shader, ShaderCompileError, shader};
use glfw::{Glfw, Window};

pub struct Renderer {
    pub(crate) glfw: Glfw,
    pub(crate) window: Window
}

pub fn new(glfw: Glfw, window: Window) -> Renderer {
    Renderer { glfw, window }
}

impl Renderer {
    pub fn shader(&mut self, vertex: &str, fragment: &str) -> Result<Shader, ShaderCompileError> {
        shader::new(vertex, fragment)
    }

    pub fn vertex_array(&mut self) -> VertexArray {
        buffer::new()
    }

    pub fn clear(&mut self, color: cgmath::Vector3<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn set_window_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height);
        unsafe { gl::Viewport(0, 0, width, height) }
    }

    pub fn get_mouse_button(&self, button: glfw::MouseButton) -> glfw::Action {
        self.window.get_mouse_button(button)
    }
}