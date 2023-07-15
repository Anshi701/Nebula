use std::ffi::{CString, CStr};
use gl::types::*;

use crate::{EGlobals::*, 
    EWinChan::win_chan::*,
    EBuffers::basic_buffers::*,
    EShaders::prog_shader::*,
};

use super::planet_generator::*;

pub struct EPlanet{
    Size: f32,
    Position: EPos,
    Triangles: Vec<Triangle>,

    shader_program: EProgram,
    vbo: EVertBuffer,
    vao: EArrayBuffer,
}

impl EPlanet{
    pub fn new(size: f32, pos: EPos) -> Self{
        let DB: PlanetGenerator = PlanetGenerator::CreatePlanet(3, size, &pos);
        let (shader_program, vbo, vao) = EPlanet::InitGPUData(DB.GetTriangles()); 
        
        Self { 
            Size: size,
            Position: pos,
            Triangles: DB.GetTriangles().clone(),
            shader_program, 
            vbo, 
            vao
        }
    }

    fn InitGPUData<T>(vertices: &Vec<T>) -> (EProgram, EVertBuffer, EArrayBuffer){
        let vert_shader = EShader::from_vert_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/Basic/triangle_vert.glsl")).unwrap()
        ).unwrap();
        
        let frag_shader = EShader::from_frag_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/Basic/triangle_frag.glsl")).unwrap()
        ).unwrap();
    
        let shader_program: EProgram = EProgram::from_shaders(
            &[vert_shader, frag_shader]
        ).unwrap();
    
        CheckGLError();
    
        let l = vertices.len();
        let sz = std::mem::size_of::<T>();
    
        //let size = (l * sz) as gl::types::GLsizeiptr;
    
        let VertBuf: EVertBuffer = EVertBuffer::new(
            gl::ARRAY_BUFFER,
            (l * sz) as gl::types::GLsizeiptr,
            gl::STATIC_DRAW, 
        );

        CheckGLError();

        VertBuf.set_data(vertices.as_slice());
    
        let ArrayBuf: EArrayBuffer = EArrayBuffer::new(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            0,
        );

        CheckGLError();

        VertBuf.bind();
        ArrayBuf.set_attribute();
        VertBuf.un_bind();

        CheckGLError();

        return (shader_program, VertBuf, ArrayBuf);
    }

    pub fn Draw(&self, channel: &EChannel){
        unsafe {           
            self.shader_program.set_used();

            self.shader_program.SetMVP(channel.camera.GetMVP().as_slice());

            self.vbo.bind();
            self.vao.bind();
        
            //gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                (self.Triangles.len() * 3) as GLsizei,             // number of indices to be rendered
            );

            self.vbo.un_bind();
            self.vao.un_bind();
        }
    }

}


