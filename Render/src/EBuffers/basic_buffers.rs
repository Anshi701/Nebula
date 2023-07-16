use gl::{self, types::*};
use std::{self, path::Path};


extern crate image;
use image::GenericImage;
//use std::ffi::{CString, CStr};

// =======================================================================
// =======================================================================
// =======================================================================

pub struct EVertBuffer{
    id: GLuint,
    target: GLuint,
    size: GLsizeiptr,
    usage: GLuint,
    
}

impl EVertBuffer {
    pub fn new(target: GLuint, size: GLsizeiptr, usage: GLuint) -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); } 
        Self { id, target, size, usage }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target, self.id); } 
    }

    pub fn un_bind(&self) {
        unsafe { gl::BindBuffer(self.target, 0); }
    }

    pub fn set_data<D>(&self, data: &[D]) {
        self.bind();
        unsafe {
            //let (_, data_bytes, _) = data.align_to::<u8>();
            let s = data.len() * std::mem::size_of::<D>();
            let dataPtr = data.as_ptr();
            gl::BufferData(
                self.target,
                s as GLsizeiptr,
                dataPtr as *const _,
                self.usage,
            );
        }
        self.un_bind();
    }

}

impl Drop for EVertBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

// =======================================================================
// =======================================================================
// =======================================================================

pub struct EVertAttrib{
    attr_index      : GLuint,
    attr_size       : GLint,
    attr_type       : GLenum,
    attr_isNormalize: GLboolean,
    attr_stride     : GLsizei,
    attr_offset     : GLint,
}

impl EVertAttrib {
    pub fn new (
        attr_index      : GLuint,
        attr_size       : GLint,
        attr_type       : GLenum,
        attr_isNormalize: GLboolean,
        attr_stride     : GLsizei,
        attr_offset     : GLint,
    ) -> Self{
        Self { attr_index, attr_size, attr_type, attr_isNormalize, attr_stride, attr_offset }
    }
}

pub struct EArrayBuffer{
    id: GLuint,
    Attribs: Vec<EVertAttrib>
    
}

impl EArrayBuffer {

    pub fn new(
        attribs: Vec<EVertAttrib>
        // attr_index      : GLuint,
        // attr_size       : GLint,
        // attr_type       : GLenum,
        // attr_isNormalize: GLboolean,
        // attr_stride     : GLsizei,
        // attr_offset     : GLint

    ) -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        Self { id , Attribs: attribs, }
    }


    pub fn bind(&self) {
        unsafe { 
            gl::BindVertexArray(self.id); 
        }
    }

    pub fn un_bind(&self) {
        unsafe { 
            gl::BindVertexArray(0); 
        }
    }


    pub fn set_attribute(
        &self
    ) {
        self.bind();
        unsafe{
            for attrib in &self.Attribs{

                gl::VertexAttribPointer(
                    attrib.attr_index, 
                    attrib.attr_size, 
                    attrib.attr_type, 
                    attrib.attr_isNormalize, 
                    attrib.attr_stride, 
                    attrib.attr_offset as *const _
                    // attrib_pos,
                    // components,
                    // gl::FLOAT,
                    // gl::FALSE,
                    // std::mem::size_of::<V>() as GLint,
                    // offset as *const _,
                );
                gl::EnableVertexAttribArray(attrib.attr_index);
            }

            
        }
    }
}

impl Drop for EArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

// =======================================================================
// =======================================================================
// =======================================================================

pub struct ETexture{
    Path: String,
    TextureID: GLuint,
    TexUnit: GLenum,
}

impl ETexture {
    pub fn new(path: &str, unit: GLenum) -> ETexture{
        //gl::TEXTURE0

        /*
            unsigned int texture;
            glGenTextures(1, &texture);
            glBindTexture(GL_TEXTURE_2D, texture);
            // set the texture wrapping/filtering options (on the currently bound texture object)
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);	
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            // load and generate the texture
            int width, height, nrChannels;
            unsigned char *data = stbi_load("container.jpg", &width, &height, &nrChannels, 0);
            if (data)
            {
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, data);
                glGenerateMipmap(GL_TEXTURE_2D);
            }
            else
            {
                std::cout << "Failed to load texture" << std::endl;
            }
            stbi_image_free(data);

         */

        let mut tex_id: u32 = 0;
        let pathToImg = Path::new(path);
        if pathToImg.exists(){
                
            unsafe{
                gl::GenTextures(1, &mut tex_id);
                gl::BindTexture(gl::TEXTURE_2D, tex_id);
                // set the texture wrapping/filtering options (on the currently bound texture object)
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                    // "../Resources/Textures/container.jpg"
                
                //let img = image::open(&pathToImg).expect("Failed to load texture");

                // let img = image::open(&pathToImg).unwrap();
                // let img = match img {
                //     image::DynamicImage::ImageRgb8(img) => img,
                //     x => x.to_rgb8()
                // };

                // let data = img.as_raw();
                // let dataPtr = data.as_ptr();
                // gl::TexImage2D(gl::TEXTURE_2D,
                //             0,
                //             gl::RGB as i32,
                //             img.width() as i32,
                //             img.height() as i32,
                //             0,
                //             gl::RGB,
                //             gl::UNSIGNED_BYTE,
                //             dataPtr as *const _);

                let img = image::open(path).expect("Failed to load texture");
                let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
                let width = img.width();
                let height = img.height();
                let data = img.into_raw();
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _,
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);

    
            }
        }
        else{
            println!("Image path {path} does not exist!")
        }
        Self{
            Path: path.to_string(), TextureID: tex_id, TexUnit: unit
        }
    }

    pub fn bindTex(&self){
        unsafe{
            gl::ActiveTexture(self.TexUnit);
            gl::BindTexture(gl::TEXTURE_2D, self.TextureID);
        }
    }

    pub fn un_bindTex(&self){
        unsafe{
            gl::ActiveTexture(self.TexUnit);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn GetUnitAsInt(&self) -> i32{
        self.TexUnit as i32 - gl::TEXTURE0 as i32
    }

}



