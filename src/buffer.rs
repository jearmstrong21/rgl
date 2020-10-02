use gl::types::*;

pub trait BufferType {
    const STRIDE: i32;
    fn flatten(self) -> Vec<f32>;
}

impl BufferType for f32 {
    const STRIDE: i32 = 1;

    fn flatten(self) -> Vec<f32> {
        vec![self]
    }
}

impl BufferType for cgmath::Vector2<f32> {
    const STRIDE: i32 = 2;

    fn flatten(self) -> Vec<f32> {
        vec![self.x, self.y]
    }
}

impl BufferType for cgmath::Vector3<f32> {
    const STRIDE: i32 = 3;

    fn flatten(self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

#[derive(Debug)]
pub struct VertexArray {
    id: GLuint,
    buffers: Vec<GLuint>,
    index_count: usize,
    ebo: GLuint,
}

pub fn new() -> VertexArray {
    unsafe {
        let mut id = 0;
        gl::GenVertexArrays(1, &mut id);
        gl::BindVertexArray(id);
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        VertexArray { id, buffers: vec![], index_count: 0, ebo }
    }
}

impl VertexArray {
    pub fn add_buffer(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            self.buffers.push(id);
            gl::BindVertexArray(0);
        }
    }

    pub fn set_buffer<T: BufferType>(&mut self, index: usize, data: Vec<T>) {
        let data = data.into_iter().flat_map(|v| v.flatten()).collect::<Vec<f32>>();
        unsafe {
            gl::BindVertexArray(self.id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffers[index]);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                           &data[0] as *const f32 as *const std::ffi::c_void,
                           gl::STATIC_DRAW);

            gl::VertexAttribPointer(index as u32, T::STRIDE, gl::FLOAT, gl::FALSE, T::STRIDE * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(index as u32);
            gl::BindVertexArray(0);
        }
    }

    pub fn set_indices(&mut self, data: Vec<i32>) {
        self.index_count = data.len();
        unsafe {
            gl::BindVertexArray(self.id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                           &data[0] as *const i32 as *const std::ffi::c_void,
                           gl::STATIC_DRAW);
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(gl::TRIANGLES, self.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}