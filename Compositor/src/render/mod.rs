use gl::types::{GLenum, GLint, GLuint};
use gl::{BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use nalgebra::{Matrix4, Vector2, Vector3};
use std::ffi::CString;
use std::ptr::null;
use crate::math;
use crate::render::api::framebuffer::FrameBuffer;
use crate::render::api::init_gl;
use crate::render::api::texture::{Texture, TextureData};
use crate::window::window::{Window, WINDOW_PADDING};
use crate::common::ScreenSize;
use crate::render::text_renderer::TextRenderer;
use crate::render::util::colour::Colour;
use crate::render::util::rect::Rect;

pub mod api;
pub mod util;
mod text_renderer;

pub struct Renderer {
    mouse_layer: FrameBuffer,
    window_layer: FrameBuffer,
    background_layer: FrameBuffer,
    text_renderer: TextRenderer,
    vao: u32,
    textured_sp: u32,
    textured_flip_sp: u32,
    colour_sp: u32,
    width: ScreenSize,
    height: ScreenSize,
    in_buffer: bool,
    view: Matrix4<f32>,
    project: Matrix4<f32>,
}


impl Renderer {
    pub fn new(width: ScreenSize, height: ScreenSize) -> Self {
        init_gl();
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };

        let textured_sp = compile_shaders(
            include_str!("shaders/textured.vert"),
            include_str!("shaders/textured.frag"),
        )
        .expect("Failed to compile texture shaders");

        let textured_flip_sp = compile_shaders(
            include_str!("shaders/textured.vert"),
            include_str!("shaders/textured_flip.frag"),
        )
            .expect("Failed to compile texture flipped shaders");

        let colour_sp = compile_shaders(
            include_str!("shaders/colour.vert"),
            include_str!("shaders/colour.frag"),
        )
        .expect("Failed to compile colour shader");
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        Self {
            mouse_layer: FrameBuffer::opengl(width, height),
            window_layer: FrameBuffer::opengl(width, height),
            background_layer: FrameBuffer::opengl(width, height),
            text_renderer: TextRenderer::new(),
            vao,
            textured_sp,
            textured_flip_sp,
            colour_sp,
            width,
            height,
            in_buffer: false,
            view: Matrix4::identity(),
            project: math::create_ortho_matrix(width as f32, height as f32),
        }
    }

    pub fn rerender_background(&mut self, texture: Texture) {
        self.in_buffer = true;
        self.background_layer.begin();
        let rect =  Rect::new(0, 0, self.width, self.height);
        self.render_rect_textured(rect, texture);
        self.mouse_layer.end();
        self.in_buffer = false;
    }

    pub fn rerender_mouse(&mut self, x: ScreenSize, y: ScreenSize, texture: Option<Texture>) {
        self.in_buffer = true;
        self.mouse_layer.begin();
        let rect =  Rect::new(x,y,20,20);
        if let Some(texture) = texture {
            self.render_rect_textured(rect, texture);
        } else {
            self.render_rect(rect,Colour::white(),false);
        }
        self.mouse_layer.end();
        self.in_buffer = false;
    }

    pub fn rerender_windows(&mut self, window: &Vec<Window>) {
        self.in_buffer = true;
        self.window_layer.begin();
        for window in window {
            if window.draw_title_bar() {
                self.render_rect(
                    Rect::new(
                        window.rect.x,
                        window.rect.y,
                        window.rect.width + WINDOW_PADDING * 2,
                        window.rect.height + 30 + WINDOW_PADDING,
                    ),
                    Colour::grayscale(32),
                    true,
                );
                let title_bar_y = window.rect.y;
                let title_bar_x = window.rect.x;
                if window.icon != Texture::none() {
                    self.render_rect(
                        Rect::new(title_bar_x + 5, title_bar_y + 5, 20, 20),
                        Colour::white(),
                        false,
                    );
                    self.render_text(
                        window.get_title().as_str(),
                        title_bar_x + 30,
                        title_bar_y + WINDOW_PADDING,
                        24f32,
                        Colour::white(),
                    );
                } else {
                    self.render_text(
                        window.get_title().as_str(),
                        title_bar_x + WINDOW_PADDING,
                        title_bar_y + WINDOW_PADDING,
                        24f32,
                        Colour::white(),
                    );
                }
            } else {
                todo!("need to render window without decorations")
            }
            self.render_rect(
                Rect::new(
                    window.rect.x + WINDOW_PADDING,
                    window.rect.y + 30,
                    window.rect.width,
                    window.rect.height,
                ),
                Colour::grayscale(64),
                false,
            );
        }
        self.window_layer.end();
        self.in_buffer = false;
    }

    pub fn render_text(&mut self, text: &str, x: ScreenSize, y: ScreenSize, size: f32, colour: Colour) {
        // Extract cache dimensions using a mutable borrow
        let (width, height) = {
            let cache = self.text_renderer.get_cached_text(text, size);
            (cache.actual_width, cache.actual_height)
        };

        // Extract the texture ID using an immutable borrow
        let texture = self.text_renderer.get_texture();

        // Create model with stored dimensions
        let model = math::create_model_matrix(
            Vector3::new(x as f32, y as f32, 0.0),
            Vector2::new(width, height),
        );

        // Render with stored values
        self.internal_render(
            model,
            Some(Vector2::new(width, height)),
            Some(colour),
            Some(texture),
            None,
        );
    }

    fn render_rect_textured(&self, rect: Rect, texture: Texture) {
        self.internal_render(
            math::create_model_matrix(
                Vector3::new(rect.x as f32, rect.y as f32, 0.0),
                Vector2::new(rect.width as f32, rect.height as f32),
            ),
            Some(Vector2::new(rect.width as f32, rect.height as f32)),
            None,
            Some(&texture),
            None,
        )
    }

    fn render_rect(&self, rect: Rect, colour: Colour, round_corner: bool) {
        let corner_radius = if round_corner { Some(0.5f32) } else { None };
        self.internal_render(
            math::create_model_matrix(
                Vector3::new(rect.x as f32, rect.y as f32, 0.0),
                Vector2::new(rect.width as f32, rect.height as f32),
            ),
            Some(Vector2::new(rect.width as f32, rect.height as f32)),
            Some(colour),
            None,
            corner_radius,
        )
    }

    fn internal_render(
        &self,
        model: Matrix4<f32>,
        size: Option<Vector2<f32>>,
        colour: Option<Colour>,
        texture: Option<&Texture>,
        corner_radius: Option<f32>,
    ) {
        unsafe {
            if let Some(tex) = texture {
                let id = if !self.in_buffer {
                    self.textured_flip_sp
                } else {
                    self.textured_sp
                };
                gl::UseProgram(id);
                gl::ActiveTexture(gl::TEXTURE0);
                match tex.data() {
                    TextureData::None => {}
                    TextureData::OPENGL(id) => {
                        gl::BindTexture(gl::TEXTURE_2D, *id);
                    }
                }
                gl::Uniform1i(7, 0);
            } else {
                gl::UseProgram(self.colour_sp);
                gl::Uniform2f(3, self.width as f32, self.height as f32);
                if let Some(colour) = colour {
                    let colour = colour.to_gl();
                    gl::Uniform4f(4, colour.x, colour.y, colour.z, colour.w);
                }
                if let Some(size) = size {
                    gl::Uniform2fv(5, 1, size.as_ptr().cast())
                }
                if let Some(corner_radius) = corner_radius {
                    gl::Uniform1f(6, corner_radius);
                } else {
                    gl::Uniform1f(6, 0.0);
                }
            };
            gl::UniformMatrix4fv(0, 1, gl::FALSE, model.as_ptr().cast());
            gl::UniformMatrix4fv(1, 1, gl::FALSE, self.view.as_ptr().cast());
            gl::UniformMatrix4fv(2, 1, gl::FALSE, self.project.as_ptr().cast());
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Enable(BLEND);
            gl::BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        }
        let rect = Rect::new(0, 0, self.width as ScreenSize, self.height as ScreenSize);
        self.render_rect_textured(rect, self.background_layer.to_texture());
        self.render_rect_textured(rect, self.window_layer.to_texture());
        self.render_rect_textured(rect, self.mouse_layer.to_texture());
        unsafe {
            gl::Disable(BLEND);
        }
    }
    pub fn cleanup(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.textured_sp);
            gl::DeleteProgram(self.colour_sp);
        }
        self.mouse_layer.cleanup();
        self.window_layer.cleanup();
        self.background_layer.cleanup();
        self.text_renderer.cleanup();
    }
}

fn compile_shader_unit(ss: &str, type_: GLenum) -> Option<GLuint> {
    // Convert the source code to a null-terminated CString
    let c_source = match CString::new(ss) {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to create CString for shader source");
            return None;
        }
    };

    // Create the shader
    let id = unsafe { gl::CreateShader(type_) };
    if id == 0 {
        println!("Failed to create shader");
        return None;
    }

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
        println!(
            "GL ERROR: {}",
            String::from_utf8_lossy(&buffer).trim_end_matches('\0')
        );

        // Delete the shader and return None
        unsafe { gl::DeleteShader(id) };
        return None;
    }

    Some(id)
}

fn compile_shaders(vss: &str, fss: &str) -> Option<GLuint> {
    if let (Some(vert_s), Some(frag_s)) = (
        compile_shader_unit(vss, gl::VERTEX_SHADER),
        compile_shader_unit(fss, gl::FRAGMENT_SHADER),
    ) {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vert_s);
            gl::AttachShader(program, frag_s);
            gl::LinkProgram(program);
        }

        let mut success = 0;
        // Check program linking
        unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };
        if success == 0 {
            let mut len = 0;
            unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buffer = vec![0u8; len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut _,
                )
            };
            println!(
                "GL ERROR: {:?}",
                String::from_utf8_lossy(&buffer).to_string()
            );
            return None;
        }
        unsafe {
            gl::DeleteShader(vert_s);
            gl::DeleteShader(frag_s);
        }
        Some(program)
    } else {
        None
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
