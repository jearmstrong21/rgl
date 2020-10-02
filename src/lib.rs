mod shader;
mod renderer;
mod buffer;

pub use shader::Shader;
pub use shader::ShaderCompileError;
pub use shader::ShaderAttachmentType;
pub use renderer::Renderer;
pub use buffer::BufferType;
pub use buffer::VertexArray;

use glfw::{WindowEvent, Window, Glfw, WindowMode, Context};
use std::sync::mpsc::Receiver;
use gl::types::*;

pub struct GL {
    glfw: Glfw,
    window: Window,
    receiver: Receiver<(f64, WindowEvent)>,
}

#[derive(Debug)]
pub struct Version<'a> {
    pub string: &'a str,
    pub major: i32,
    pub minor: i32,
}

#[derive(Debug)]
pub struct ContextInformation<'a> {
    pub version: Version<'a>,
    pub renderer: &'a str,
    pub vendor: &'a str,
    pub shader_version: &'a str,
}

impl GL {
    pub fn new() -> Result<GL, ()> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS.clone()).map_err(|_| ())?;
        let (mut window, receiver) = glfw.create_window(500, 500, "Title", WindowMode::Windowed).ok_or(())?;
        window.make_current();
        window.set_all_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        Ok(GL { glfw, window, receiver })
    }

    fn get_string(&self, value: GLenum) -> &str {
        unsafe { std::ffi::CStr::from_ptr(gl::GetString(value) as *const std::os::raw::c_char).to_str().unwrap() }
    }

    fn get_stringi(&self, value: GLenum, index: u32) -> &str {
        unsafe { std::ffi::CStr::from_ptr(gl::GetStringi(value, index) as *const std::os::raw::c_char).to_str().unwrap() }
    }

    fn get_integer(&self, value: GLenum) -> i32 {
        unsafe {
            let mut result = 0;
            gl::GetIntegerv(value, &mut result);
            result
        }
    }

    pub fn get_information(&self) -> ContextInformation {
        ContextInformation {
            version: Version {
                string: self.get_string(gl::VERSION),
                major: self.get_integer(gl::MAJOR_VERSION),
                minor: self.get_integer(gl::MINOR_VERSION),
            },
            renderer: self.get_string(gl::RENDERER),
            vendor: self.get_string(gl::VENDOR),
            shader_version: self.get_string(gl::SHADING_LANGUAGE_VERSION),
        }
    }

    pub fn get_extensions(&self) -> Vec<&str> {
        let count = self.get_integer(gl::NUM_EXTENSIONS);
        let mut extensions = Vec::with_capacity(count as usize);
        for i in 0..count {
            extensions.push(self.get_stringi(gl::EXTENSIONS, i as u32))
        }
        extensions
    }

    pub fn render_loop<S, I: Fn(&mut Renderer) -> S, F: Fn(&mut S, Vec<WindowEvent>, &mut Renderer)>(mut self, init: I, frame: F) {
        let mut renderer = renderer::new(self.glfw, self.window);
        let mut state = init(&mut renderer);
        while !renderer.window.should_close() {
            frame(&mut state, self.receiver.try_iter().map(|e| e.1).collect(), &mut renderer);
            renderer.glfw.poll_events();
            renderer.window.swap_buffers();
        }
    }
}