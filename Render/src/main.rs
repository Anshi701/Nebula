#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod EShaders;
mod EBuffers;
mod ECamera;
mod EMath;
mod EGlobals;
mod EWinChan;

use EWinChan::win_chan::*;

use std::{ffi::CString, vec, time::Duration, time::SystemTime};
use sdl2::{self, event::Event, keyboard, mouse};
use gl::types::*;

fn main() {
    let _sdl = sdl2::init().unwrap();

    let mut ewin  = EWindow::new(&_sdl, "Test", 1280, 800);
    // let mut ewin2 = EWindow::new(&_sdl, "Test", 1280, 800);
    let mut event_pump = _sdl.event_pump().unwrap();
    
    loop{
        for event in event_pump.poll_iter(){
            ewin .HandleEvent(&event);
            //ewin2.HandleEvent(&event);
        }
        let keyboard_state: keyboard::KeyboardState = sdl2::keyboard::KeyboardState::new(&event_pump);
        let mouse_state: sdl2::mouse::MouseState = sdl2::mouse::MouseState::new(&event_pump);
        ewin .HandleState(&keyboard_state, &mouse_state);
        //ewin2.HandleState(&keyboard_state, &mouse_state);

        
        ewin .drawWindow();
        //ewin2.drawWindow();
    }
}