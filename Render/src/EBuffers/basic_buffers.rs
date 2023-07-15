use gl::{self, types::*};
use std;
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

pub struct EArrayBuffer{
    id: GLuint,
    attr_index      : GLuint,
    attr_size       : GLint,
    attr_type       : GLenum,
    attr_isNormalize: GLboolean,
    attr_stride     : GLsizei,
    attr_offset     : GLint,
    
}

impl EArrayBuffer {

    pub fn new(
        attr_index      : GLuint,
        attr_size       : GLint,
        attr_type       : GLenum,
        attr_isNormalize: GLboolean,
        attr_stride     : GLsizei,
        attr_offset     : GLint

    ) -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        Self { id , attr_index, attr_size, attr_type, attr_isNormalize, attr_stride, attr_offset, }
    }


    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn un_bind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }


    pub fn set_attribute(
        &self
    ) {
        self.bind();
        unsafe{
            gl::EnableVertexAttribArray(self.attr_index);
            gl::VertexAttribPointer(
                self.attr_index, 
                self.attr_size, 
                self.attr_type, 
                self.attr_isNormalize, 
                self.attr_stride, 
                self.attr_offset as *const _
                // attrib_pos,
                // components,
                // gl::FLOAT,
                // gl::FALSE,
                // std::mem::size_of::<V>() as GLint,
                // offset as *const _,
            );
            
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
