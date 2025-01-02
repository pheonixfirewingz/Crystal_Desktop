use crate::common::ScreenSize;
use gl::types::{GLint, GLuint};
use std::ptr;

pub struct Texture {
    texture_id: GLuint,
    width: ScreenSize,
    height: ScreenSize,
    owns_texture: bool,
}

impl Texture {
    pub fn new(width: ScreenSize, height: ScreenSize) -> Self {
        unsafe {
            let mut texture = gl::INVALID_INDEX;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            Self {
                texture_id: texture,
                width,
                height,
                owns_texture: true,
            }
        }
    }
    pub fn id(&self) -> GLuint {
        self.texture_id
    }

    pub fn not_owned(texture_id: GLuint,width: ScreenSize,height: ScreenSize) -> Self {
        Self {
            texture_id,
            width,
            height,
            owns_texture: false,
        }
    }

    pub fn not_owned_from(texture: &Self) -> Self {
        Self {
            texture_id: texture.texture_id,
            width: texture.width,
            height: texture.height,
            owns_texture: false,
        }
    }

    pub fn read(&self) -> Vec<u8> {
        unsafe {
            let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buffer.as_mut_ptr() as *mut _,
            );
            buffer
        }
    }

    pub fn write(&mut self, data: &[u8], width: ScreenSize, height: ScreenSize) {
        if self.owns_texture {
            let width = if self.width == 0 {
                self.width = width;
                width
            } else {
                self.width
            };
            let height = if self.height == 0 {
                self.height = height;
                height
            } else {
                self.height
            };
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    width as i32,
                    height as i32,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _,
                );
            }
        }
    }

    pub fn width(&self) -> ScreenSize {
        self.width
    }

    pub fn height(&self) -> ScreenSize {
        self.height
    }

    pub fn cleanup(&mut self) {
        if self.owns_texture {
            unsafe {
                gl::DeleteTextures(1, &self.texture_id);
            }
        }
    }
}
