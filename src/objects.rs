use std::{
    error::Error,
    ffi::{CStr, CString},
    mem::{offset_of, size_of, size_of_val},
    ptr::{null, null_mut},
};

use gl::types::{GLchar, GLenum, GLint, GLsizeiptr, GLuint, GLvoid};

/// OpenGL Shader (Rendering Pipeline)
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
                gl::DeleteShader(id);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id());
        }
    }
}
/// OpenGL Program (A sequence of Shader calls)
pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
                gl::DeleteProgram(id);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Program { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    /// This sets the (Shader)Program as the current one e.g: gl::UseProgram(..)
    pub fn set(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id());
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = vec![b' '; len];
    buffer.push(0);
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_program() -> Result<Program, Box<dyn Error>> {
    let vert_src = std::fs::read("./src/vert.glsl")?;
    let frag_src = std::fs::read("./src/frag.glsl")?;

    let vert_c = CString::new(vert_src)?;
    let frag_c = CString::new(frag_src)?;

    let vert_shader = Shader::from_source(&vert_c, gl::VERTEX_SHADER)?;
    let frag_shader = Shader::from_source(&frag_c, gl::FRAGMENT_SHADER)?;

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader])?;

    Ok(shader_program)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: (f32, f32),
    pub color: (f32, f32, f32),
    pub tex_coord: (f32, f32),
}
impl Vertex {
    pub fn new(pos: (f32, f32), color: (f32, f32, f32), tex_coord: (f32, f32)) -> Self {
        Self {
            position: pos,
            color,
            tex_coord,
        }
    }

    /// This sets up the vertex attributes in memory that get sent to the shader
    pub fn desc() {
        let stride = size_of::<Self>();

        unsafe {
            // Vertex Position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,               // Index of the generic Vertex Attribute (layout (location=0)
                2,               // Number of components per Attribute
                gl::FLOAT,       // Data type
                gl::FALSE,       // Normalized (int-to-float conversion)
                stride as GLint, // Stride (byte offset between
                // consecutive attributes)
                offset_of!(Vertex, position) as *const GLvoid, // Offset of the First/Previous component
            );

            // Vertex Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as GLint,
                offset_of!(Vertex, color) as *const GLvoid,
            );

            // TexCoord Coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride as GLint,
                offset_of!(Vertex, tex_coord) as *const GLvoid,
            );
        }
    }
}

/// Vertex Buffer Object
pub struct Vbo {
    pub id: GLuint,
}

impl Vbo {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Vbo { id }
    }

    pub fn set(&self, data: &[Vertex]) {
        self.bind();
        self.data(data);
    }

    fn data(&self, vertices: &[Vertex]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

/// Index Buffer Object
pub struct Ibo {
    pub id: GLuint,
}

impl Ibo {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ibo { id }
    }

    pub fn set(&self, data: &[u32]) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &[u32]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(indices) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

/// Vertex Array Object
pub struct Vao {
    pub id: GLuint,
}

impl Vao {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        Vertex::desc();
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

/// Uniform Object
pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: u32, name: &str) -> Result<Self, String> {
        let cname = CString::new(name).expect("CString::new failed in Uniform Creation");
        let location: GLint = unsafe { gl::GetUniformLocation(program, cname.as_ptr()) };
        if location == -1 {
            return Err(format!("Couldn't get Uniform location for {}", name));
        }
        Ok(Uniform { id: location })
    }

    pub fn set_1f(&self, value: f32) {
        unsafe {
            gl::Uniform1f(self.id, value);
        }
    }

    pub fn set_vec2f(&self, value: (f32, f32)) {
        unsafe {
            gl::Uniform2f(self.id, value.0, value.1);
        }
    }
}
