use gl::{self, types::*};
use std;
use std::ffi::{CString, CStr};
use sdl2::{self, event::Event, keyboard, mouse, video::Window};

//mod planet_generator;
use super::planet_generator::*;

use crate::ECamera::camera::*;
use crate::EGlobals::*;
use crate::EShaders::prog_shader::*;
use crate::EBuffers::basic_buffers::*;


//use crate::gl_buffer::*;
//use crate::renderGL::*;
//use crate::globals::*;

struct EViewPort{
    x: u32,
    y: u32,
    width: u32, 
    height: u32,
}

pub struct EWindow<'a>{
    sdl: &'a sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    //event_pump: sdl2::EventPump,
    win: Window,
    gl_context: sdl2::video::GLContext,

    channel: EChannel,

    isCaptured: bool,
    isWireframe: bool,
    
}

impl<'a> EWindow<'a>{
    pub fn new(_sdl: &'a sdl2::Sdl, _title: &str, _width: u32, _height: u32)->Self{
        //let _sdl: sdl2::Sdl = sdl2::init().unwrap();
        let _video_subsystem: sdl2::VideoSubsystem = _sdl.video().unwrap();
        let _gl_attr = _video_subsystem.gl_attr();
        _gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        _gl_attr.set_context_version(4, 5);
        let _window: sdl2::video::Window = _video_subsystem
            .window(_title, _width, _height)
            .opengl() // add opengl flag
            .resizable()
            .build()
            .unwrap();
        //let _event_pump: sdl2::EventPump = _sdl.event_pump().unwrap();
        let _gl_context: sdl2::video::GLContext = _window.gl_create_context().unwrap();
        gl::load_with(|s: &str| _video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        CheckGLError();

        Self{
            sdl: _sdl,
            video_subsystem: _video_subsystem,
            //event_pump: _event_pump,
            win: _window,
            gl_context: _gl_context,

            channel: EChannel::new(0, 0, _width, _height),

            isCaptured: false,
            isWireframe: false,
        }
        
    }

    pub fn HandleEvent(&mut self, event: & sdl2::event::Event, ){
        
        match event {
            Event::Quit {..} => return,
            Event::KeyDown {
                keycode: Some(keyboard::Keycode::Escape), .. 
            } => { return },
            Event::KeyDown {
                keycode: Some(keyboard::Keycode::C), .. 
            } => { 
                if self.isCaptured { self.isCaptured = false; self.sdl.mouse().show_cursor(true); }
                else { self.isCaptured = true; self.sdl.mouse().show_cursor(false);} 
            },

            Event::KeyDown {
                keycode: Some(keyboard::Keycode::V), .. 
            } => { 
                if self.isWireframe { self.isWireframe = false; }
                else { self.isWireframe = true; } 

                
            },
            
            _ => {},
        }
        
    }

    pub fn HandleState(&mut self, keyboard_state: & keyboard::KeyboardState, mouse_state: & mouse::MouseState){
        match self.sdl.mouse().focused_window_id() {

            Some(id) => {
                

                if id == self.win.id(){
                    //sdl.mouse().capture(true);
                    if self.isCaptured{

                        //let keyboard_state: keyboard::KeyboardState = sdl2::keyboard::KeyboardState::new(&event_pump);
                        self.channel.camera.HandleKeyboard(&keyboard_state);

                        //let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
                        let (width, height) = self.win.size();
                        let x = mouse_state.x();
                        let y = mouse_state.y();
                        self.channel.camera.HandleMousePos(x as f32, y as f32, width as f32, height as f32);
                        self.sdl.mouse().warp_mouse_in_window(&self.win, (width/2) as i32, (height/2) as i32);
                    }
                    //mouse_state.
                }
            }
            None => {}
        }
    }

    pub fn drawWindow(&mut self){
        self.win.gl_set_context_to_current().unwrap();
        //self.gl_context

        unsafe{
            
            if self.isWireframe { gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE ); }
            else {gl::PolygonMode( gl::FRONT_AND_BACK, gl::FILL );}
        }

        CheckGLError();

        self.channel.drawChannel();

        self.win.gl_swap_window();
    }
}


pub struct EChannel{
    id: u32,
    viewport: EViewPort,
    camera: ECamera,

    shader_program: EProgram,
    vbo: EVertBuffer,
    vao: EArrayBuffer,

    DB: PlanetGenerator,

}

impl EChannel{
    pub fn new(x: u32, y: u32, width: u32,  height: u32,)->Self{
        let mut cam: ECamera = ECamera::new();

        cam.SetPosXYZ(6030., 0., 0.0);
        cam.SetAngles(0., 0., 0.);

        let (DB, shader_program, vbo, vao) = EChannel::InitData();

        Self { id: 0, viewport: EViewPort { x, y, width, height } , camera: cam, 
            shader_program,
            vbo,
            vao,
            DB,
        }
    }

    fn InitData()-> (PlanetGenerator, EProgram, EVertBuffer, EArrayBuffer){

        let DB: PlanetGenerator = PlanetGenerator::CreatePlanet(3);
        let (shader_program, vbo, vao) = EChannel::setup(DB.GetTriangles()); 
        
        (DB, shader_program, vbo, vao)
    }

    fn setup<T>(vertices: &Vec<T>) -> (EProgram, EVertBuffer, EArrayBuffer){
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

        //shader_program.set_used();
    
        // set up vertex buffer object
    
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
        // set up shared state for window
    
        unsafe {
            gl::Viewport(0, 0, 1280, 800);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
    
        return (shader_program, VertBuf, ArrayBuf);
    }

    pub fn drawChannel(&mut self){
        self.camera.CalcMatrices();

        CheckGLError();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            self.shader_program.set_used();

            self.shader_program.SetMVP(self.camera.GetMVP().as_slice());

            CheckGLError();


            CheckGLError();

            self.vbo.bind();
            self.vao.bind();

            CheckGLError();
        
            //gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                self.DB.GetTrianglesCount() as GLsizei,             // number of indices to be rendered
            );

            CheckGLError();

            self.vbo.un_bind();
            self.vao.un_bind();

            gl::Finish();
        }
    }
}

