use std::vec;
//use crate::camera::{RAD2DEG, DEG2RAD};
use crate::EGlobals::*;
use crate::EMath::emath::*;

//use gl::MapBufferRange;




#[derive(Clone)]
pub enum TileQuater {
    TopLeft,
    TopRight,
    BotRight, 
    BotLeft,
}


#[derive(Copy, Clone)]
pub struct MapPoint{
    x: f32,
    y: f32,

}

// impl MapPoint{
//     pub fn Interp(&self, point: &MapPoint, value: f32) -> MapPoint{
//         let mut dx = point.x - self.x;
//         let mut dy = point.y - self.y;

//         let l = value * (dx*dx + dy*dy).sqrt();
//         dx = dx * l;
//         dy = dy * l;

//         MapPoint { x: dx, y: dy }
//     }
    
// }


#[derive(Clone)]
pub struct Tile{
    Lvl: u32,
    Quater: TileQuater,
    P1: MapPoint,
    P2: MapPoint,
    P3: MapPoint,
    P4: MapPoint,
    //Path: String,
    //Vertices: Vec<Vertex>
    Triangles: Vec<Triangle>,
}

impl Tile{
    pub fn new (
        Lvl : u32,
        Quater: TileQuater,
        P1  : MapPoint,
        P2  : MapPoint,
        P3  : MapPoint,
        P4  : MapPoint,
        size: &f32, offset: &EPos
    ) -> Self{
        let mut tile = Self { Lvl, Quater, P1, P2, P3, P4, Triangles: Vec::<Triangle>::new() };
        tile.MakeTileTriangles(tile.P1, tile.P3, size, offset);
        tile
    }

    pub fn MakeTileTriangles(&mut self, p1: MapPoint, p2: MapPoint, size: &f32, offset: &EPos){
        let mesh_size = TILE_DIM as f32;
        
        if !self.Triangles.is_empty() {return;}

        //let mut tri_vec = Vec::<Triangle>::new();
        
        let TopLeftBig = MapPoint{x: p1.x, y: p1.y};
        //let TopRighBig = MapPoint{x: p2.x, y: p1.y};
        //let BotLeftBig = MapPoint{x: p1.x, y: p2.y};
        //let BotRighBig = MapPoint{x: p2.x, y: p2.y};
    
        let mut TopLeft = MapPoint{x: 0., y: 0.};
        let mut TopRigh = MapPoint{x: 0., y: 0.};
        let mut BotLeft = MapPoint{x: 0., y: 0.};
        let mut BotRigh = MapPoint{x: 0., y: 0.};
    
        let step_x = (p2.x - p1.x) / mesh_size;
        let step_y = (p2.y - p1.y) / mesh_size;
    
        let mut m ;
        let mut n ;
    
        let mut v1 = Vertex{ x: 0., y: 0., z: 0. };
        let mut v2 = Vertex{ x: 0., y: 0., z: 0. };
        let mut v3 = Vertex{ x: 0., y: 0., z: 0. };
        //let mut t  = Triangle{v1, v2, v3};
    
        for i in 0..(TILE_DIM){
            for j in 0..(TILE_DIM){
                m = j as f32;
                n = i as f32;
    
                TopLeft.x = TopLeftBig.x + step_x * (m - 0.);
                TopLeft.y = TopLeftBig.y + step_y * (n - 0.);
    
                TopRigh.x = TopLeftBig.x + step_x * (m + 1.);
                TopRigh.y = TopLeftBig.y + step_y * (n - 0.);
    
                BotLeft.x = TopLeftBig.x + step_x * (m - 0.);
                BotLeft.y = TopLeftBig.y + step_y * (n + 1.);
    
                BotRigh.x = TopLeftBig.x + step_x * (m + 1.);
                BotRigh.y = TopLeftBig.y + step_y * (n + 1.);
    
                v1.x = TopLeft.x; v1.y = TopLeft.y; 
                v2.x = TopRigh.x; v2.y = TopRigh.y; 
                v3.x = BotLeft.x; v3.y = BotLeft.y; 
    
                self.Triangles.push(Triangle {  
                    v1: v1.ToXYZ().ToRadius(size).ToOffsetPos(offset), 
                    v2: v2.ToXYZ().ToRadius(size).ToOffsetPos(offset), 
                    v3: v3.ToXYZ().ToRadius(size).ToOffsetPos(offset) 
                });
    
                v1.x = TopRigh.x; v1.y = TopRigh.y; 
                v2.x = BotRigh.x; v2.y = BotRigh.y; 
                v3.x = BotLeft.x; v3.y = BotLeft.y; 
    
                self.Triangles.push(Triangle {  
                    v1: v1.ToXYZ().ToRadius(size).ToOffsetPos(offset), 
                    v2: v2.ToXYZ().ToRadius(size).ToOffsetPos(offset), 
                    v3: v3.ToXYZ().ToRadius(size).ToOffsetPos(offset) 
                });
    
            }
        }
    
        
    }
}

pub struct PlanetGenerator<>{
    GenerationLevel: u32,
    All_tiles: Vec<Tile>,
    All_triangles: Vec<Triangle>,

    counter: i32,
    counter1: i32,
    counter2: i32,
    counter3: i32,
    counter4: i32,

    size: f32, 
    offset: EPos
}

impl PlanetGenerator{


    pub fn CreatePlanet(gen_lvl: u32, size: f32, offset: &EPos) -> Self{
        let All_tiles = Vec::<Tile>::new();
        let All_triangles = Vec::<Triangle>::new();
        
        let mut Planet = Self {GenerationLevel: gen_lvl, All_tiles, All_triangles, size, offset: offset.clone(),
                                                counter: 0 , counter1:0, counter2:0, counter3:0, counter4:0};
        let PlanetRef = &mut Planet;

        let semisphere_left = Tile::new( 
             1, 
             TileQuater::TopLeft,
             MapPoint{x: -SCALE_LON, y:  SCALE_LAT},
             MapPoint{x:  SCALE_CNT, y:  SCALE_LAT},
             MapPoint{x:  SCALE_CNT, y: -SCALE_LAT},
             MapPoint{x: -SCALE_LON, y: -SCALE_LAT},
             &PlanetRef.size, &PlanetRef.offset
        );

        let semisphere_right = Tile::new( 
            1, 
            TileQuater::TopRight,
            MapPoint{x:  SCALE_CNT, y:  SCALE_LAT},
            MapPoint{x:  SCALE_LON, y:  SCALE_LAT},
            MapPoint{x:  SCALE_LON, y: -SCALE_LAT},
            MapPoint{x:  SCALE_CNT, y: -SCALE_LAT},
            &PlanetRef.size, &PlanetRef.offset
        );

        PlanetRef.CreateTree(semisphere_left);
        PlanetRef.CreateTree(semisphere_right);
        
        Planet
    }

    pub fn GetTriangles(&self) -> &Vec<Triangle>{
        &self.All_triangles
    }

    pub fn GetTrianglesCount(&self) -> i32{
        (self.All_triangles.len() * 3) as i32
    }

    fn AddTile(&mut self, tile: Tile){
        //tile.Triangles.clone()
        self.counter += 1;
        if tile.Lvl != self.GenerationLevel {
            //let a = 0;
            return;
        }
        self.All_triangles.append(&mut tile.Triangles.clone());
        self.All_tiles.push(tile);
    }

    fn CreateTree(&mut self, tile: Tile){
        if tile.Lvl < self.GenerationLevel {
            let child1 = self.CreateChild(&tile, TileQuater::TopLeft ).unwrap();
            let child2 = self.CreateChild(&tile, TileQuater::TopRight).unwrap();
            let child3 = self.CreateChild(&tile, TileQuater::BotRight).unwrap();
            let child4 = self.CreateChild(&tile, TileQuater::BotLeft ).unwrap();
    
            self.CreateTree(child1);
            self.CreateTree(child2);
            self.CreateTree(child3);
            self.CreateTree(child4);
        }


        match tile.Quater{
            TileQuater::TopLeft  => {
                self.counter1 += 1;
                
            },
            TileQuater::TopRight => {
                self.counter2 += 1;
                
            },
            TileQuater::BotRight => {
                self.counter3 += 1;

            },
            TileQuater::BotLeft  => {
                self.counter4 += 1;
            },
        }
        

        self.AddTile(tile);
    }

    fn CreateChild(&mut self, parent: &Tile, quater: TileQuater) -> Option<Tile>{
        
        let Center = MapPoint{
            x: parent.P3.x + (parent.P1.x - parent.P3.x) / 2.,
            y: parent.P3.y + (parent.P1.y - parent.P3.y) / 2.,
        };
        let TopMiddle = MapPoint{
            x: parent.P2.x + (parent.P1.x - parent.P2.x) / 2.,
            y: parent.P2.y + (parent.P1.y - parent.P2.y) / 2.,
        };
        let RightMiddle = MapPoint{
            x: parent.P3.x + (parent.P2.x - parent.P3.x) / 2.,
            y: parent.P3.y + (parent.P2.y - parent.P3.y) / 2.,
        };
        let BotMiddle = MapPoint{
            x: parent.P4.x + (parent.P3.x - parent.P4.x) / 2.,
            y: parent.P4.y + (parent.P3.y - parent.P4.y) / 2.,
        };
        let LeftMiddle = MapPoint{
            x: parent.P1.x + (parent.P4.x - parent.P1.x) / 2.,
            y: parent.P1.y + (parent.P4.y - parent.P1.y) / 2.,
        };

        let p1;
        let p2;
        let p3;
        let p4;

        match quater{
            TileQuater::TopLeft  => {
                //self.counter1 += 1;
                p1 = parent.P1.clone();
                p2 = TopMiddle.clone();
                p3 = Center.clone();
                p4 = LeftMiddle.clone();
            },
            TileQuater::TopRight => {
                //self.counter2 += 1;
                p1 = TopMiddle.clone();
                p2 = parent.P2.clone();
                p3 = RightMiddle.clone();
                p4 = Center.clone();
            },
            TileQuater::BotRight => {
                //self.counter3 += 1;
                p1 = Center.clone();
                p2 = RightMiddle.clone();
                p3 = parent.P3.clone();
                p4 = BotMiddle.clone();
            },
            TileQuater::BotLeft  => {
                //self.counter4 += 1;
                p1 = LeftMiddle.clone();
                p2 = Center.clone();
                p3 = BotMiddle.clone();
                p4 = parent.P4.clone();
            },
        }
        let child_lvl = parent.Lvl + 1;

        let child: Tile = Tile::new( 
            child_lvl, 
            quater,
            p1,
            p2,
            p3,
            p4,
            &self.size, &self.offset
       );
       Some(child)


    }
}


