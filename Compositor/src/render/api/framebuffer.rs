use crate::render::api::texture::{Texture, TextureData};
use gl::types::{GLint, GLsizei, GLuint};
use gl::*;
use std::ptr::null;
use crate::common::ScreenSize;

pub struct OpenGLFramebuffer {
    pub texture: GLuint,
    framebuffer: GLuint,
}

pub enum FrameBufferData {
    None,
    OPENGL(OpenGLFramebuffer),
}

pub struct FrameBuffer {
    data: FrameBufferData,
    width:ScreenSize,
    height: ScreenSize,
}

impl FrameBuffer {
    pub fn none() -> Self {
        Self {
            data: FrameBufferData::None,
            width: 0,
            height: 0,
        }
    }
    pub fn opengl(width: ScreenSize, height: ScreenSize) -> Self {
        unsafe {
            let mut framebuffer: GLuint = INVALID_INDEX;
            let mut texture: GLuint = 0;

            // Generate framebuffer and bind it
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(FRAMEBUFFER, framebuffer);

            // Create texture for framebuffer
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(TEXTURE_2D, texture);
            gl::TexImage2D(
                TEXTURE_2D,
                0,
                RGBA as GLint, // Internal format with alpha
                width as GLsizei,
                height as GLsizei,
                0,
                RGBA, // Format includes alpha
                UNSIGNED_BYTE,
                null(),
            );

            // Texture parameters for transparency and edge handling
            gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
            gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
            gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as GLint);

            // Attach texture to framebuffer
            gl::FramebufferTexture2D(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, texture, 0);

            // Ensure framebuffer completeness
            let status = gl::CheckFramebufferStatus(FRAMEBUFFER);
            if status != FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete: {}", status);
            }
            gl::ClearColor(0f32,0f32,0f32,0f32);
            gl::Clear(COLOR_BUFFER_BIT);
            gl::BindFramebuffer(FRAMEBUFFER, 0);

            Self {
                data: FrameBufferData::OPENGL(OpenGLFramebuffer {
                    texture,
                    framebuffer,
                }),
                width,
                height,
            }
        }
    }

    pub fn begin(&self) {
        match &self.data {
            FrameBufferData::None => {}
            FrameBufferData::OPENGL(opengl_framebuffer) => unsafe {
                gl::BindFramebuffer(DRAW_FRAMEBUFFER, opengl_framebuffer.framebuffer);
                gl::ClearColor(0f32,0f32,0f32,0f32);
                gl::Clear(COLOR_BUFFER_BIT);

                // Enable blending for transparency
                gl::Enable(BLEND);
                gl::BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            },
        }
    }

    pub fn end(&self) {
        match &self.data {
            FrameBufferData::None => {}
            FrameBufferData::OPENGL(_) => unsafe {
                gl::Disable(BLEND); // Disable blending after rendering
                gl::BindFramebuffer(DRAW_FRAMEBUFFER, 0); // Unbind framebuffer
            },
        }
    }

    pub fn to_texture(&self) -> Texture {
        match &self.data {
            FrameBufferData::None => Texture::none(),
            FrameBufferData::OPENGL(opengl_framebuffer) => Texture::new(
                TextureData::OPENGL(opengl_framebuffer.texture),
                self.width,
                self.height,
            ),
        }
    }

    pub fn cleanup(&self) {
        match &self.data {
            FrameBufferData::None => {}
            FrameBufferData::OPENGL(opengl_framebuffer) => unsafe {
                gl::DeleteFramebuffers(1, &opengl_framebuffer.framebuffer);
                gl::DeleteTextures(1, &opengl_framebuffer.texture);
            },
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        self.cleanup();
    }
}