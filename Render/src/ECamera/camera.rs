use crate::EMath::emath::*;
use sdl2::{self, event::Event, keyboard};
use nalgebra as na;
use na::Vector3;

pub struct ECamera{
    X: f32,
    Y: f32,
    Z: f32,

    Yaw     :    f32,
    Pitch   :  f32,
    Roll     :   f32,

    Speed: f32,

    Right: na::Vector3<f32>,
    Forward: na::Vector3<f32>,
    Up: na::Vector3<f32>,

    LastMousePosX: f32,
    LastMousePosY: f32,

    ModelViewMatr: na::Matrix4<f32>,
    MVP: na::Matrix4<f32>,
    
}

impl ECamera{
    pub fn new() -> Self{

        let Speed: f32 = 5.0; 

        Self { 
            X: 0.0,
            Y: 0.0,
            Z: 0.0,

            Yaw:   0.0,
            Pitch: 0.0,
            Roll:  0.0,

            Speed,

            Right: na::Vector3::<f32>::zeros(),
            Forward: na::Vector3::<f32>::zeros(),
            Up: na::Vector3::<f32>::zeros(),

            LastMousePosX: 0.,
            LastMousePosY: 0.,

            ModelViewMatr: na::Matrix4::zeros(),
            MVP: na::Matrix4::zeros()
         }
    }

    pub fn GetMVP(&self)->na::Matrix4<f32>{
        self.MVP
    }

    pub fn HandleKeyboard(&mut self, kayboard_state: &keyboard::KeyboardState){
        let mut scale: f32 = 1.0;
        if kayboard_state.is_scancode_pressed(keyboard::Scancode::LShift){ scale = 10.0; }

        if kayboard_state.is_scancode_pressed(keyboard::Scancode::W){ self.MoveByVec(self.Forward,  self.Speed * scale); }
        if kayboard_state.is_scancode_pressed(keyboard::Scancode::A){ self.MoveByVec(self.Right  , -self.Speed * scale); }
        if kayboard_state.is_scancode_pressed(keyboard::Scancode::S){ self.MoveByVec(self.Forward, -self.Speed * scale); }
        if kayboard_state.is_scancode_pressed(keyboard::Scancode::D){ self.MoveByVec(self.Right  ,  self.Speed * scale); }            
            
    }

    pub fn HandleMousePos(&mut self, x: f32, y: f32, win_w: f32, win_h: f32){
        let diff_x = x - win_w/2.0;
        let diff_y = y - win_h/2.0;

        self.Yaw   += diff_x / 10.;
        self.Pitch += diff_y / 10.;

        self.LastMousePosX = x;
        self.LastMousePosY = y;
    }

    fn MoveByVec(&mut self, v: Vector3<f32>, speed: f32){
        self.X += v.x * speed;
        self.Y += v.y * speed;
        self.Z += v.z * speed;
    }

    pub fn SetPosXYZ(&mut self, X: f32, Y: f32, Z: f32,){
        self.X = X;
        self.Y = Y;
        self.Z = Z;
    }

    pub fn SetAngles(&mut self, Yaw: f32, Pitch: f32, Roll: f32,){
        self.Yaw   = Yaw   ;
        self.Pitch = Pitch ;
        self.Roll  = Roll  ;
        // println!("New Yaw: {Yaw}, Pitch: {Pitch}, Roll: {Roll}")
    }

    pub fn CalcMatrices(&mut self){
 
        //let base = CreateBaseGeoMatr( self.X, self.Y, self.Z);
        let base = CreateBaseMatr();
        let transl = GetTranslationMatrix(self.X, self.Y, self.Z);
        let rot = GetRotationMatrix(self.Yaw, self.Pitch, self.Roll);
        let ModelView = rot * base * transl;
        
        let proj = GetProjectionMatrix(16.0 / 9.0, 60.0 * DEG2RAD, 1.0, 1_000_000.0);
        let MVP = proj * ModelView ;

        self.ModelViewMatr = ModelView;
        self.MVP = MVP;

        self.ExtractBasis(self.ModelViewMatr);

    }


    fn ExtractBasis(&mut self, matr: na::Matrix4<f32>){
        self.Right      = Vector3::new(matr.m11, matr.m12, matr.m13).normalize();
        self.Up         = Vector3::new(matr.m21, matr.m22, matr.m23).normalize();
        self.Forward    = Vector3::new(-matr.m31, -matr.m32, -matr.m33).normalize();
    }

    fn GetVec4(&self)->na::Vector4<f32>{
        na::Vector4::new(self.X, self.Y, self.Z, 1.)
    }

}