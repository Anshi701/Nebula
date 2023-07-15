
use gl::{self, types::*};
use std;
use std::ffi::{CString, CStr};


// =======================================================================
// =======================================================================
// =======================================================================

pub struct EProgram {
    id: GLuint,
}

impl EProgram {
    pub fn from_shaders(shaders: &[EShader]) -> Result<EProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id); }
        }

        unsafe { gl::LinkProgram(program_id); }

        // continue with error handling here

        //unsafe {
        //    let  matrLoc = gl::GetUniformLocation(program_id, GetCStr("MVP").as_ptr());
        //    gl::UniformMatrix4fv(matrLoc, 1, gl::FALSE, glm::value_ptr(model));
        //}


        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id); }
        }

        Ok(EProgram { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    unsafe fn GetLocation(&self, name: &str)-> GLint{
        gl::GetUniformLocation(self.id, GetCStr(name).as_ptr())
    }

    pub fn SetMVP(&self, matr: &[f32]){
        unsafe {
            let loc: GLint = self.GetLocation("MVP");
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, matr.as_ptr());
        }

    }
    
}

impl Drop for EProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

// =======================================================================
// =======================================================================
// =======================================================================

pub struct EShader {
    id: gl::types::GLuint,
}

impl EShader {
    pub fn from_source(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<EShader, String> {
        let id = EShader::shader_from_source(source, kind)?;
        Ok(EShader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<EShader, String> {
        EShader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<EShader, String> {
        EShader::from_source(source, gl::FRAGMENT_SHADER)
    }

    fn shader_from_source(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
    
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
    
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
    
            let error = create_whitespace_cstring_with_len(len as usize);
    
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
    
            return Err(error.to_string_lossy().into_owned());
        }
    
        Ok(id)
    }
    
}

impl Drop for EShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// =======================================================================
// =======================================================================
// =======================================================================

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn GetCStr(s: &str)->CString{
    CString::new(s).unwrap()
}
//}