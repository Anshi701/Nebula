use nalgebra as na;
use na::Vector3;





pub const DEG2RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / std::f32::consts::PI;

pub const TILE_DIM: u8 = 64;
 
pub const SCALE_LAT: f32 = 90.;
pub const SCALE_LON: f32 = 180.;
pub const SCALE_CNT: f32 = 0.;
 
pub const WGS84_RAD_EQ: f32 = 6_378_137.;
pub const WGS84_RAD_PL: f32 = 6_356_752.;
pub const EARTH_RAD: f32 = 6_371_000. / 1000.0 - 371.0;



pub fn XYZtoLLA(x: f32, y: f32, z: f32) -> (f32,f32,f32){
    let alt: f32 = (x*x + y*y + z*z).sqrt() - EARTH_RAD         ;
    let lat: f32 = (z / (alt + EARTH_RAD)).acos()                * RAD2DEG;
    let lon: f32 = y.signum() * (x / (x*x + y*y).sqrt()).acos()  * RAD2DEG;

    (lat, lon, alt)
}

pub fn LLAtoXYZ(lat: f32, lon: f32, alt: f32) -> (f32,f32,f32){
    let x: f32 = (alt + EARTH_RAD) * (lat * DEG2RAD).sin() * (lon * DEG2RAD).cos();
    let y: f32 = (alt + EARTH_RAD) * (lat * DEG2RAD).sin() * (lon * DEG2RAD).sin();
    let z: f32 = (alt + EARTH_RAD) * (lat * DEG2RAD).cos();

    (x, y, z)
}

pub fn dot(v1: na::Vector3<f32>, v2: na::Vector3<f32>) -> f32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn GetTranslationMatrix(X: f32, Y: f32, Z: f32,) -> na::Matrix4<f32>{
    na::Matrix4::new(
        1., 0.,0., -X,
        0., 1.,0., -Y,
        0., 0.,1., -Z,
        0., 0.,0., 1.,
    )
}

pub fn GetRotationMatrix(Yaw: f32, Pitch: f32, Roll: f32,) -> na::Matrix4<f32>{
    let rot1 = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), Yaw   * DEG2RAD);
    let rot2 = na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), Pitch * DEG2RAD);
    let rot3 = na::Rotation3::from_axis_angle(&na::Vector3::z_axis(), Roll  * DEG2RAD);
    RotToMat4(rot3) * RotToMat4(rot2) * RotToMat4(rot1)
}

pub fn GetProjectionMatrix(aspect: f32, fovy: f32, znear: f32, zfar: f32)-> na::Matrix4<f32>{
    na::Perspective3::new(aspect, fovy, znear, zfar).into_inner()
}

pub fn CreateBaseMatr() -> na::Matrix4<f32>{

    let right = na::Vector3::new(1.,  0., 0.);
    let up    = na::Vector3::new(0.,  0., 1.);
    let back  = na::Vector3::new(0., -1., 0.);

    na::Matrix4::new( 
        right.x,   right.y,   right.z, 0.,
        up.x, up.y, up.z, 0.,
        back.x, back.y, back.z, 0.,
        0., 0., 0., 1.,
    )
}

pub fn CreateBaseGeoMatr(X: f32, Y: f32, Z: f32,) -> na::Matrix4<f32>{
    let axisZ = na::Vector3::new(0., 0., 1.);
    let up    = na::Vector3::new(X, Y, Z).normalize();
    if up.x == axisZ.x && up.y == axisZ.y && up.z == axisZ.z {
        return CreateBaseMatr();
    }
    let right = axisZ.cross(&up).normalize();
    let back  = right.cross(&up).normalize();
    
    let ax_x = right;  //axisX; right;
    let ax_y = up;     //axisY; up;   
    let ax_z = back;   //axisZ; back; 
    //let pos = na::Vector3::new(X, Y, Z);
    na::Matrix4::new( 
        ax_x.x, ax_y.x, ax_z.x, 0.0,
        ax_x.y, ax_y.y, ax_z.y, 0.0,
        ax_x.z, ax_y.z, ax_z.z, 0.0,
        0., 0., 0., 1.,
    ).transpose()
}

pub fn RotToMat4(rot: na::Rotation<f32, 3>) -> na::Matrix4<f32>{
    Mat3ToMat4(*rot.matrix())
}

pub fn Mat3ToMat4(matr: na::Matrix3<f32>) -> na::Matrix4<f32>{
    //let m = na::Matrix4::<f32>::identity();
    let mut matr = matr.fixed_resize::<4, 4>(0.0);
    matr.m44 = 1.0;
    matr
}