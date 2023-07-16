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
    MapTex: ETexture,

    shader_program: EProgram,
    VertexBuffer: EVertBuffer,
    VertexArrayBuffer: EArrayBuffer,
    //ArrayBuffers: Vec<EArrayBuffer>,
}

impl EPlanet{
    pub fn new(size: f32, pos: EPos) -> Self{
        let DB: PlanetGenerator = PlanetGenerator::CreatePlanet(3, size, &pos);
        let (shader_program, vbo, vao, mapTex) = EPlanet::InitGPUData(DB.GetTriangles()); 
        
        Self { 
            Size: size,
            Position: pos,
            Triangles: DB.GetTriangles().clone(),
            MapTex: mapTex,
            shader_program, 
            VertexBuffer: vbo, 
            VertexArrayBuffer: vao
            //ArrayBuffers: vaos
        }
    }

    fn InitGPUData<T>(vertices: &Vec<T>) -> (EProgram, EVertBuffer, EArrayBuffer, ETexture){

        // ############################### SHADERS ################################################
        let vert_shader = EShader::from_vert_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/Basic/planet_vert.glsl")).unwrap()
        ).unwrap();
        
        let frag_shader = EShader::from_frag_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/Basic/planet_frag.glsl")).unwrap()
        ).unwrap();
    
        let shader_program: EProgram = EProgram::from_shaders(
            &[vert_shader, frag_shader]
        ).unwrap();
    
        CheckGLError();

        // ############################### TEXTURES ################################################
        //let texMap: ETexture = ETexture::new("../Resources/Textures/EarthMap.jpeg", gl::TEXTURE0);
        let texMap: ETexture = ETexture::new("C:/__Files/Projects/Taktika_Galactica/Render/src/Resources/Textures/EarthMap.jpeg", gl::TEXTURE0);
        CheckGLError();
        shader_program.set_used();
        shader_program.SetTextureUnit("planetMap", texMap.GetUnitAsInt());
        CheckGLError();

        // ############################### BUFFERS ################################################
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
    
        // =================================
        let attrSize_Position = (3 * std::mem::size_of::<f32>()) as gl::types::GLint;
        let attrStride =        (5 * std::mem::size_of::<f32>()) as gl::types::GLint;
        //let attrSize_UV = (2 * std::mem::size_of::<f32>()) as gl::types::GLint;

        let attrOffset_UV = attrSize_Position;

        let Attrib_Position: EVertAttrib = EVertAttrib::new(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            attrStride, // stride (byte offset between consecutive attributes)
            0,
        );

        let Attrib_UV: EVertAttrib = EVertAttrib::new(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            attrStride, // stride (byte offset between consecutive attributes)
            attrOffset_UV,
        );

        let mut attrVec = Vec::<EVertAttrib>::new();
        attrVec.push(Attrib_Position);
        attrVec.push(Attrib_UV);

        let ArrayBuf: EArrayBuffer = EArrayBuffer::new(attrVec);
        CheckGLError();

        VertBuf.bind();
        ArrayBuf.set_attribute();
        VertBuf.un_bind();

        CheckGLError();

        // ###############################################################################

        return (shader_program, VertBuf, ArrayBuf, texMap);
    }

    pub fn Draw(&self, channel: &EChannel){
        unsafe {           
            self.MapTex.bindTex();
            self.shader_program.set_used();

            self.shader_program.SetMVP(channel.camera.GetMVP().as_slice());

            self.VertexBuffer.bind();
            self.VertexArrayBuffer.bind();
            // for vao in &self.ArrayBuffers{
            //     vao.bind();
            // }
        
            //gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                (self.Triangles.len() * 3) as GLsizei,             // number of indices to be rendered
            );

            self.VertexBuffer.un_bind();
            self.VertexArrayBuffer.un_bind();
            //for vao in &self.ArrayBuffers{
                //    vao.un_bind();
                //}
            //self.MapTex.un_bindTex();
        }
    }

}


