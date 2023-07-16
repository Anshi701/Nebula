
use gl::{self, types::*}; 
use crate::EMath::emath::*;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct EPos{
    pub X: f32, 
    pub Y: f32, 
    pub Z: f32,
}

impl EPos {
    pub fn new(x: f32, y: f32, z: f32) -> EPos{
        Self { X: x, Y: y, Z: z }  
    }

    pub fn zero() -> EPos{
        Self { X: 0.0, Y: 0.0, Z: 0.0 }  
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub U: f32,
    pub V: f32,
}

impl Vertex{
    pub fn ToXYZ(self) -> Self{
        let (x, y, z) = LLAtoXYZ(self.x, self.y, self.z);
        Vertex { x, y, z, U: self.U, V: self.V}
    }

    pub fn ToRadius(mut self, newR: &f32)-> Self{
        let oldR = self.GetRadius();
        self.x = self.x * (newR / oldR);
        self.y = self.y * (newR / oldR);
        self.z = self.z * (newR / oldR);
        self
    }

    pub fn ToOffsetPos(mut self, nPos: &EPos) -> Self{
        self.x = self.x + nPos.X;
        self.y = self.y + nPos.Y;
        self.z = self.z + nPos.Z;
        self
    }

    pub fn ToOffsetXYZ(mut self, nX: &f32, nY: &f32, nZ: &f32,) -> Self{
        self.x = self.x + nX;
        self.y = self.y + nY;
        self.z = self.z + nZ;
        self
    }

    pub fn ToLLA(self) -> Self{
        let (lat, lon, alt) = XYZtoLLA(self.x, self.y, self.z);
        Vertex { x:lat, y:lon, z:alt, U: self.U, V: self.V}
    }

    pub fn GetRadius(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Triangle{
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}


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
