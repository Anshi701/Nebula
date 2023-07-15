
use gl::{self, types::*}; 


pub fn CheckGLError(){
    unsafe{

        let errorCode: GLenum  = gl::GetError();
        if errorCode != gl::NO_ERROR
        {
            let mut error_str: String = "".to_string();

            match errorCode {
                gl::INVALID_ENUM                  => {error_str = "INVALID_ENUM"                 .to_string();}
                gl::INVALID_VALUE                 => {error_str = "INVALID_VALUE"                .to_string();}
                gl::INVALID_OPERATION             => {error_str = "INVALID_OPERATION"            .to_string();}
                gl::STACK_OVERFLOW                => {error_str = "STACK_OVERFLOW"               .to_string();}
                gl::STACK_UNDERFLOW               => {error_str = "STACK_UNDERFLOW"              .to_string();}
                gl::OUT_OF_MEMORY                 => {error_str = "OUT_OF_MEMORY"                .to_string();}
                gl::INVALID_FRAMEBUFFER_OPERATION => {error_str = "INVALID_FRAMEBUFFER_OPERATION".to_string();}
                _ => {}
            }
            
            //let string_slice1 = &error_str[..];
            //let string_slice2 = &error_str;
            //let string_slice3 = &*error_str;
 
            println!("{error_str}");
        }
    }

}
