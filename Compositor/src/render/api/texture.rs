use crate::common::ScreenSize;
use gl::types::{GLint, GLuint};
use std::any::Any;
use std::ptr;

pub enum TextureData {
    None,
    OPENGL(GLuint),
}

pub struct Texture {
    data: TextureData,
    width: ScreenSize,
    height: ScreenSize,
    owns_texture: bool,
}

impl Texture {
    #[inline]
    pub fn new(data: TextureData, width: ScreenSize, height: ScreenSize) -> Self {
        Self {
            data,
            width,
            height,
            owns_texture: false,
        }
    }

    #[inline]
    pub fn none() -> Self {
        Self {
            data: TextureData::None,
            width: 0,
            height: 0,
            owns_texture: true,
        }
    }

    pub fn opengl(width: ScreenSize, height: ScreenSize) -> Self {
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
                data: TextureData::OPENGL(texture),
                width,
                height,
                owns_texture: true,
            }
        }
    }

    pub fn read(&self) -> Vec<u8> {
        match &self.data {
            TextureData::None => vec![],
            TextureData::OPENGL(opengl) => unsafe {
                let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
                gl::BindTexture(gl::TEXTURE_2D, *opengl);
                gl::GetTexImage(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    buffer.as_mut_ptr() as *mut _,
                );
                buffer
            },
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
            match &self.data {
                TextureData::None => {}
                TextureData::OPENGL(opengl) => unsafe {
                    gl::BindTexture(gl::TEXTURE_2D, *opengl);
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
                },
            }
        }
    }

    pub fn width(&self) -> ScreenSize {
        self.width
    }

    pub fn height(&self) -> ScreenSize {
        self.height
    }

    pub fn data(&self) -> &TextureData {
        &self.data
    }
    
    pub fn cleanup(&mut self) {
        if let TextureData::OPENGL(texture) = &self.data {
            if self.owns_texture {
                unsafe {
                    gl::DeleteTextures(1, texture);
                }
            }
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.data.type_id() == other.data.type_id()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.cleanup();
    }
}
