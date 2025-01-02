use gl::types::{GLenum, GLint, GLuint};
use std::ffi::CString;
use std::ptr::null;

pub struct ShaderProgram {
    program_id: GLuint,
}

impl ShaderProgram {
    pub fn new(vert_src:&str,frag_src:&str) -> Self {
        let shader_program_id = unsafe { gl::CreateProgram() };
        let vert_id = compile_shader_unit(vert_src, gl::VERTEX_SHADER);
        let frag_id = compile_shader_unit(frag_src, gl::FRAGMENT_SHADER);
        unsafe {
            gl::AttachShader(shader_program_id, vert_id);
            gl::AttachShader(shader_program_id, frag_id);
            gl::LinkProgram(shader_program_id);
        }

        let mut success = 0;
        // Check program linking
        unsafe { gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success) };
        if success == 0 {
            let mut len = 0;
            unsafe { gl::GetProgramiv(shader_program_id, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buffer = vec![0u8; len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    shader_program_id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut _,
                )
            };
            panic!(
                "GL ERROR: {:?}",
                String::from_utf8_lossy(&buffer).to_string()
            );
        }
        unsafe {
            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);
        }
        Self {
            program_id: shader_program_id,
        }
    }
    
    pub fn activate(&self) { 
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
    
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.cleanup();
    }
}

fn compile_shader_unit(ss: &str, type_: GLenum) -> GLuint {
    // Convert the source code to a null-terminated CString
    let c_source = match CString::new(ss) {
        Ok(s) => s,
        Err(_) => {
            panic!("Failed to create CString for shader source");
        }
    };
    // Create the shader
    let id = unsafe { gl::CreateShader(type_) };
    // Set the shader source
    unsafe { gl::ShaderSource(id, 1, &c_source.as_ptr(), null()) };
    // Compile the shader
    unsafe { gl::CompileShader(id) };
    // Check for compilation errors
    let mut success = gl::FALSE as GLint;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };
    if success != gl::TRUE as GLint {
        let mut len = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) };
        let mut buffer = vec![0u8; len as usize];
        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut _)
        };
        // Delete the shader and return None
        unsafe { gl::DeleteShader(id) };
        panic!(
            "GL ERROR: {}",
            String::from_utf8_lossy(&buffer).trim_end_matches('\0')
        );
    }
    id
}