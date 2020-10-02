use gl::types::*;
use std::ffi::CString;
use cgmath::Matrix;

pub trait Uniform {
    unsafe fn set(&self, location: GLint);
}

impl Uniform for f32 {
    unsafe fn set(&self, location: GLint) {
        gl::Uniform1f(location, *self);
    }
}

impl Uniform for cgmath::Vector2<f32> {
    unsafe fn set(&self, location: GLint) {
        gl::Uniform2f(location, self.x, self.y);
    }
}

impl Uniform for cgmath::Vector3<f32> {
    unsafe fn set(&self, location: GLint) {
        gl::Uniform3f(location, self.x, self.y, self.z);
    }
}

impl Uniform for cgmath::Vector4<f32> {
    unsafe fn set(&self, location: GLint) {
        gl::Uniform4f(location, self.x, self.y, self.z, self.w);
    }
}

impl Uniform for cgmath::Matrix4<f32> {
    unsafe fn set(&self, location: GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr())
    }
}

#[derive(Debug)]
pub struct Shader(GLuint);

pub struct Uniforms(GLuint); // private field

impl Uniforms {
    pub fn set<U: Uniform>(&mut self, name: &str, value: &U) {
        unsafe { value.set(gl::GetUniformLocation(self.0, CString::new(name).unwrap().as_ptr())) }
    }
}

impl Shader {
    pub fn with<F: FnOnce(&mut Uniforms)>(&self, f: F) {
        unsafe { gl::UseProgram(self.0); }
        f(&mut Uniforms(self.0));
        unsafe { gl::UseProgram(0); }
    }
}

#[derive(Debug)]
pub enum ShaderAttachmentType {
    Vertex,
    Fragment,
}

#[derive(Debug)]
pub enum ShaderCompileError {
    InvalidShaderCode(ShaderAttachmentType),
    InvalidShaderError(ShaderAttachmentType),
    InvalidLinkError,
    Compile(ShaderAttachmentType, String),
    Link(String),
}

pub fn new(vertex: &str, fragment: &str) -> Result<Shader, ShaderCompileError> {
    unsafe {
        let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex = CString::new(vertex.as_bytes()).map_err(|_| ShaderCompileError::InvalidShaderCode(ShaderAttachmentType::Vertex))?;
        gl::ShaderSource(vertex_id, 1, &vertex.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_id);
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_id, gl::COMPILE_STATUS, &mut success);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_id, 512, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            return Err(ShaderCompileError::Compile(ShaderAttachmentType::Vertex, String::from_utf8(info_log).map_err(|_| ShaderCompileError::InvalidShaderError(ShaderAttachmentType::Vertex))?));
        }

        let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment = CString::new(fragment.as_bytes()).map_err(|_| ShaderCompileError::InvalidShaderCode(ShaderAttachmentType::Fragment))?;
        gl::ShaderSource(fragment_id, 1, &fragment.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_id);
        gl::GetShaderiv(fragment_id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_id, 512, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            return Err(ShaderCompileError::Compile(ShaderAttachmentType::Fragment, String::from_utf8(info_log).map_err(|_| ShaderCompileError::InvalidShaderError(ShaderAttachmentType::Fragment))?));
        }

        let program_id = gl::CreateProgram();
        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);
        gl::LinkProgram(program_id);
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(program_id, 512, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            return Err(ShaderCompileError::Link(String::from_utf8(info_log).map_err(|_| ShaderCompileError::InvalidLinkError)?));
        }
        gl::DeleteShader(vertex_id);
        gl::DeleteShader(fragment_id);
        Ok(Shader(program_id))
    }
}