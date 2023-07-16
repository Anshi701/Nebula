use gl::{self, types::*};
use std;
use std::ffi::{CString, CStr};
use sdl2::{self, event::Event, keyboard, mouse, video::Window};

//mod planet_generator;
//use super::planet_generator::*;

use crate::ECamera::camera::*;
use crate::EGlobals::*;
use crate::EShaders::prog_shader::*;
use crate::EBuffers::basic_buffers::*;
use crate::EPlanetSystem::EPlanetSystem::*;


//use crate::gl_buffer::*;
//use crate::renderGL::*;
//use crate::globals::*;

struct EViewPort{
    x: i32,
    y: i32,
    width: i32, 
    height: i32,
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

            channel: EChannel::new(0, 0, _width as i32, _height as i32),

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

        self.channel.DrawObjects();

        unsafe{
            gl::Finish();
        }

        self.win.gl_swap_window();
    }
}


pub struct EChannel{
    id: u32,
    viewport: EViewPort,
    pub camera: ECamera,
    planetSystem: EPlanetSystem,
    CoordSystem: ECoordinateSystem
    // shader_program: EProgram,
    // vbo: EVertBuffer,
    // vao: EArrayBuffer,

    //DB: PlanetGenerator,

}

impl EChannel{
    pub fn new(x: i32, y: i32, width: i32,  height: i32,)->Self{
        let mut cam: ECamera = ECamera::new();

        cam.SetPosXYZ(1500., 0., 0.0);
        cam.SetAngles(0., 0., 0.);

        let planetSys = EPlanetSystem::new();
        let CoordSystem = ECoordinateSystem::new();

        Self { id: 0, viewport: EViewPort { x, y, width, height } , camera: cam, 
            planetSystem: planetSys, CoordSystem
            // shader_program,
            // vbo,
            // vao,
            // DB,
        }
    }

    pub fn DrawObjects(&mut self){
        self.SetupViewport();
        self.camera.CalcMatrices();
        self.DrawCoordinateSystem();
        self.planetSystem.Draw(&self);

    }

    fn SetupViewport(&self){
        unsafe {
            gl::Viewport(self.viewport.x, self.viewport.y, self.viewport.width, self.viewport.height);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS );

        }
    }

    fn DrawCoordinateSystem(&self){
        self.CoordSystem.Draw(self);
    }

    
}

pub struct CoordVertex{
    x: f32,
    y: f32, 
    z: f32, 
    r: f32, 
    g: f32, 
    b: f32,
}

pub struct ECoordinateSystem{
    shader_program: EProgram,
    VertexBuffer: EVertBuffer,
    ArrayBuf: EArrayBuffer,
}

impl ECoordinateSystem {
    pub fn new () -> ECoordinateSystem{
        

        // ############################### SHADERS ################################################
        let vert_shader = EShader::from_vert_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/CoordSys/coordSys_vert.glsl")).unwrap()
        ).unwrap();
        
        let frag_shader = EShader::from_frag_source(
            &CString::new(include_str!("../Resources/ShaderSrcs/CoordSys/coordSys_frag.glsl")).unwrap()
        ).unwrap();
    
        let shader_program: EProgram = EProgram::from_shaders(
            &[vert_shader, frag_shader]
        ).unwrap();
    
        CheckGLError();

        // ############################### BUFFERS ################################################

        // 4 points == 4 * (3 * 4) ==
        let mut vertices = Vec::<CoordVertex>::new();
        vertices.push(CoordVertex{x:        0.0, y:       0.0, z:       0.0, r: 1.0, g: 0.0, b: 0.0});
        vertices.push(CoordVertex{x:  1000000.0, y:       0.0, z:       0.0, r: 1.0, g: 0.0, b: 0.0});

        vertices.push(CoordVertex{x:        0.0, y:       0.0, z:       0.0, r: 0.0, g: 1.0, b: 0.0});
        vertices.push(CoordVertex{x:        0.0, y: 1000000.0, z:       0.0, r: 0.0, g: 1.0, b: 0.0});
        
        vertices.push(CoordVertex{x:        0.0, y:       0.0, z:       0.0, r: 0.0, g: 0.0, b: 1.0});
        vertices.push(CoordVertex{x:        0.0, y:       0.0, z: 1000000.0, r: 0.0, g: 0.0, b: 1.0});

        let POINT_COUNT = vertices.len();
        let coordSize = std::mem::size_of::<CoordVertex>();
        let sz = POINT_COUNT * coordSize;

        let VertexBuffer: EVertBuffer = EVertBuffer::new(
            gl::ARRAY_BUFFER,
            (sz) as gl::types::GLsizeiptr,
            gl::STATIC_DRAW, 
        ); 

        VertexBuffer.set_data(vertices.as_slice());

        let attrStride =        (6 * std::mem::size_of::<f32>()) as gl::types::GLint;
        let attrSize_Position = (3 * std::mem::size_of::<f32>()) as gl::types::GLint;
        //let attrSize_UV = (2 * std::mem::size_of::<f32>()) as gl::types::GLint;

        let attrOffset_Color = attrSize_Position;

        let Attrib_Position: EVertAttrib = EVertAttrib::new(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            attrStride, // stride (byte offset between consecutive attributes)
            0,
        );

        let Attrib_Color: EVertAttrib = EVertAttrib::new(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            attrStride, // stride (byte offset between consecutive attributes)
            attrOffset_Color,
        );

        let mut attrVec = Vec::<EVertAttrib>::new();
        attrVec.push(Attrib_Position);
        attrVec.push(Attrib_Color);

        let ArrayBuf: EArrayBuffer = EArrayBuffer::new(attrVec);
        CheckGLError();

        VertexBuffer.bind();
        ArrayBuf.set_attribute();
        VertexBuffer.un_bind();

        CheckGLError();

        Self { shader_program, VertexBuffer, ArrayBuf }

    }

    pub fn Draw(&self, channel: &EChannel){
        unsafe{

            self.shader_program.set_used();
            self.shader_program.SetMVP(channel.camera.GetMVP().as_slice());

            self.VertexBuffer.bind();
            self.ArrayBuf.bind();
            gl::DrawArrays(
                gl::LINES, // mode
                0,             // starting index in the enabled arrays
                (6) as GLsizei,             // number of indices to be rendered
            );

            self.VertexBuffer.un_bind();
            self.ArrayBuf.un_bind();
        }
    }

}