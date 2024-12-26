use crate::render::framebuffer;
use gl::types::{GLint, GLuint};
use glfw::{Action, Context, Key};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,RwLock};

const VERT_SHADER: &'static str = include_str!("./shaders/vs.glsl");
const FRAG_SHADER: &'static str = include_str!("./shaders/fs.glsl");

unsafe fn create_texture(width: u32, height: u32) -> GLuint {
    let mut texture_id: GLuint = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as GLint,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as GLint,
    );
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as GLint,
        width as GLint,
        height as GLint,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        ptr::null(),
    );
    gl::BindTexture(gl::TEXTURE_2D, 0);
    texture_id
}

unsafe fn create_fullscreen_quad() -> (GLuint, GLuint) {
    // Vertex data for a fullscreen quad
    let vertices: [f32; 24] = [
        // Positions       // Texture Coords
        -1.0, -1.0,        0.0, 0.0,  // Bottom-left
        1.0, -1.0,        1.0, 0.0,  // Bottom-right
        -1.0,  1.0,        0.0, 1.0,  // Top-left

        -1.0,  1.0,        0.0, 1.0,  // Top-left
        1.0, -1.0,        1.0, 0.0,  // Bottom-right
        1.0,  1.0,        1.0, 1.0,  // Top-right
    ];

    let mut vao = 0;
    let mut vbo = 0;

    // Create VAO and VBO
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    // Position attribute
    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Texture coordinate attribute
    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        4 * std::mem::size_of::<f32>() as i32,
        (2 * std::mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(1);

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    (vao, vbo)
}

unsafe fn create_shader_program() -> GLuint {
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let vert_shader_size: gl::types::GLint = VERT_SHADER.len() as gl::types::GLint;
    gl::ShaderSource(vertex_shader, 1, &VERT_SHADER.as_ptr().cast(), &vert_shader_size);
    gl::CompileShader(vertex_shader);
    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let frag_shader_size: gl::types::GLint = FRAG_SHADER.len() as gl::types::GLint;
    gl::ShaderSource(
        fragment_shader,
        1,
        &FRAG_SHADER.as_ptr().cast(),
        &frag_shader_size
    );
    gl::CompileShader(fragment_shader);
    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
    program
}

pub fn start_debug_screen(fb: Arc<RwLock<framebuffer::FrameBuffer>>, shutdown: Arc<AtomicBool>) {
    std::thread::spawn( move || {
        let fb = Arc::clone(&fb);
        let width: u32 =  {
            let guard = fb.read().expect("Failed to read frame buffer");
            guard.width()
        };
        let height: u32 = {
            let guard = fb.read().expect("Failed to read frame buffer");
            guard.height()
        };
        let shutdown = shutdown.clone();
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW init error");
        // Request OpenGL 4.6 Core Profile
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                "Debug Screen",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.make_current();
        gl::load_with(|s| window.get_proc_address(s));
        window.set_key_polling(true);

        

        // Set OpenGL viewport
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        // Create shader program
        let shader_program = unsafe { create_shader_program() };

        // Create texture
        let texture = unsafe { create_texture(width, height) };

        // Create fullscreen quad
        let (vao, vbo) = unsafe { create_fullscreen_quad() };

        // Main render loop
        while !window.should_close() && !shutdown.load(Ordering::Relaxed) {
            // Poll for and process events
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true);
                        shutdown.store(true, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }

            unsafe {
                // Clear the screen
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // Use the shader program
                gl::UseProgram(shader_program);

                // Update texture data
                let guard = fb.read().expect("Failed to read frame buffer");
                let c = guard.clone();
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    width as GLint,
                    height as GLint,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    guard.as_ptr() as *const _,
                );

                // Bind the VAO and draw the quad
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                // Unbind the VAO
                gl::BindVertexArray(0);
            }

            // Swap front and back buffers
            window.swap_buffers();
        }

        // Cleanup
        shutdown.store(true, Ordering::Relaxed);
        unsafe {
            gl::DeleteProgram(shader_program);
            gl::DeleteTextures(1, &texture);
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteVertexArrays(1, &vao);
        }
    });
}

