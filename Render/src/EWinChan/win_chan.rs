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

        Self { id: 0, viewport: EViewPort { x, y, width, height } , camera: cam, 
            planetSystem: planetSys,
            // shader_program,
            // vbo,
            // vao,
            // DB,
        }
    }

    pub fn DrawObjects(&mut self){
        self.SetupViewport();
        self.camera.CalcMatrices();
        self.planetSystem.Draw(&self);

    }

    fn SetupViewport(&self){
        unsafe {
            gl::Viewport(self.viewport.x, self.viewport.y, self.viewport.width, self.viewport.height);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    
}

