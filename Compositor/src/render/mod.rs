use crate::common::ScreenSize;
use crate::render::api::framebuffer::FrameBuffer;
use crate::render::api::init_gl;
use crate::render::api::shaderprogram::ShaderProgram;
use crate::render::api::texture::Texture;
use crate::render::util::colour::Colour;
use crate::render::util::rect::Rect;
use crate::window::window::{WINDOW_PADDING, Window};
use crate::{common, math};
use gl::{BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use nalgebra::{Matrix4, Vector2, Vector3};
use std::collections::HashMap;

pub mod api;
pub mod util;
#[repr(C, align(16))]
pub struct Renderer {
    mouse_layer: FrameBuffer,
    window_layer: FrameBuffer,
    background_layer: FrameBuffer,
    vao: u32,
    textured_sp: ShaderProgram,
    textured_flip_sp: ShaderProgram,
    colour_sp: ShaderProgram,
    width: ScreenSize,
    height: ScreenSize,
    in_buffer: bool,
    view: Matrix4<f32>,
    project: Matrix4<f32>,
    screen_rect: Rect,
}

impl Renderer {
    pub fn new(width: ScreenSize, height: ScreenSize) -> Self {
        init_gl();
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        let vertex_shader = common::file::read_from_usr_share("shaders/share.vert");
        let texture_f_shader = common::file::read_from_usr_share("shaders/textured.frag");
        let texture_flip_f_shader = common::file::read_from_usr_share("shaders/textured_flip.frag");
        let colour_f_shader = common::file::read_from_usr_share("shaders/colour.frag");
        Self {
            mouse_layer: FrameBuffer::new(width, height),
            window_layer: FrameBuffer::new(width, height),
            background_layer: FrameBuffer::new(width, height),
            vao,
            textured_sp: ShaderProgram::new(vertex_shader.as_str(), texture_f_shader.as_str()),
            textured_flip_sp: ShaderProgram::new(
                vertex_shader.as_str(),
                texture_flip_f_shader.as_str(),
            ),
            colour_sp: ShaderProgram::new(vertex_shader.as_str(), colour_f_shader.as_str()),
            width,
            height,
            in_buffer: false,
            view: Matrix4::identity(),
            project: Matrix4::new_orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 1.0),
            screen_rect: Rect::new(0, 0, width, height),
        }
    }

    #[inline(always)]
    pub fn rerender_background(&mut self, texture: &Texture) {
        self.in_buffer = true;
        self.background_layer.begin();
        self.render_rect_textured(&self.screen_rect, texture);
        self.mouse_layer.end();
        self.in_buffer = false;
    }

    #[inline(always)]
    pub fn rerender_mouse(&mut self, x: ScreenSize, y: ScreenSize) {
        self.in_buffer = true;
        self.mouse_layer.begin();
        self.render_rect(&Rect::new(x - 1, y - 2, 15, 15), Colour::green());
        self.mouse_layer.end();
        self.in_buffer = false;
    }

    #[inline(always)]
    pub fn rerender_windows(&mut self, windows: &HashMap<u64, Window>) {
        self.in_buffer = true;
        self.window_layer.begin();
        for window in windows.values() {
            if !window.is_minimized() {
                let rect = window.get_render_rect();
                if window.has_title_bar() {
                    self.render_rect(
                        &Rect::new(
                            rect.position.x,
                            rect.position.y,
                            rect.size.width + WINDOW_PADDING * 2,
                            rect.size.height + 30 + WINDOW_PADDING,
                        ),
                        Colour::grayscale_alpha(32, 225),
                    );
                    let title_bar_y = rect.position.y;
                    let title_bar_x = rect.position.x;
                    if window.has_icon() {
                        self.render_rect(
                            &Rect::new(title_bar_x + 5, title_bar_y + 5, 20, 20),
                            Colour::white(),
                        );
                    }
                }
                self.render_rect(
                    &Rect::new(
                        rect.position.x + WINDOW_PADDING,
                        rect.position.y + 30,
                        rect.size.width,
                        rect.size.height,
                    ),
                    Colour::grayscale(64),
                );
            } else if window.is_maximized() {
                self.render_rect(&self.screen_rect, Colour::grayscale(64));
            }
        }
        self.window_layer.end();
        self.in_buffer = false;
    }

    #[inline(always)]
    fn render_rect_textured(&self, rect: &Rect, texture: &Texture) {
        self.internal_texture(
            math::create_model_matrix(
                Vector3::new(rect.position.x as f32, rect.position.y as f32, 0.0),
                Vector2::new(rect.size.width as f32, rect.size.height as f32),
            ),
            texture,
        )
    }

    #[inline(always)]
    fn render_rect(&self, rect: &Rect, colour: Colour) {
        self.internal_colour(
            math::create_model_matrix(
                Vector3::new(rect.position.x as f32, rect.position.y as f32, 0.0),
                Vector2::new(rect.size.width as f32, rect.size.height as f32),
            ),
            colour,
        )
    }

    #[inline(always)]
    fn internal_colour(&self, model: Matrix4<f32>, colour: Colour) {
        self.colour_sp.activate();
        let colour = colour.to_gl();
        unsafe {
            gl::UniformMatrix4fv(0, 1, gl::FALSE, model.as_ptr().cast());
            gl::UniformMatrix4fv(1, 1, gl::FALSE, self.view.as_ptr().cast());
            gl::UniformMatrix4fv(2, 1, gl::FALSE, self.project.as_ptr().cast());
            gl::Uniform4f(3, colour.x, colour.y, colour.z, colour.w);
            gl::Uniform2fv(
                4,
                1,
                Vector2::new(self.width as f32, self.height as f32)
                    .as_ptr()
                    .cast(),
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    #[inline(always)]
    fn internal_texture(&self, model: Matrix4<f32>, texture: &Texture) {
        if !self.in_buffer {
            self.textured_flip_sp.activate()
        } else {
            self.textured_sp.activate();
        };
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id());
            gl::UniformMatrix4fv(0, 1, gl::FALSE, model.as_ptr().cast());
            gl::UniformMatrix4fv(1, 1, gl::FALSE, self.view.as_ptr().cast());
            gl::UniformMatrix4fv(2, 1, gl::FALSE, self.project.as_ptr().cast());
            gl::Uniform1i(3, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    #[inline(always)]
    pub fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Enable(BLEND);
            gl::BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        }
        let rect = Rect::new(0, 0, self.width as ScreenSize, self.height as ScreenSize);
        self.render_rect_textured(&rect, &self.background_layer.to_texture());
        self.render_rect_textured(&rect, &self.window_layer.to_texture());
        self.render_rect_textured(&rect, &self.mouse_layer.to_texture());
        unsafe {
            gl::Disable(BLEND);
        }
    }

    pub fn cleanup(&mut self) {
        self.colour_sp.cleanup();
        self.textured_sp.cleanup();
        self.textured_flip_sp.cleanup();
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        self.mouse_layer.cleanup();
        self.window_layer.cleanup();
        self.background_layer.cleanup();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
