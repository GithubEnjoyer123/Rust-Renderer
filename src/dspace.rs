use core::fmt;
use std::ops::{Add, Mul, Sub, Div};

pub type Point3d = Vec3;
pub type Color = Vec3;

//struct vector for oop use
//Derive is basically making a copy of our "class" so you can reference to self and new as you construct objects
#[derive(Clone, Copy)]
pub struct Vec3{
    pub x: f64,
   pub y: f64,
    pub z: f64
}


impl Vec3 {
    //A public constructor for our vector (they are private by default in rust)
    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3 {x, y, z}
    }

    //Function for addition
    pub fn add_vec(&self, other: &Vec3) -> Vec3{
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    //Function for subtraction
    pub fn sub_vec(&self, other: &Vec3) -> Vec3{
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    
    //Function for multiplication
    pub fn mult_vec(&self, other: &Vec3) -> Vec3{
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }

    //function for division (might be incorrect, will have to test later)
    pub fn div_vec(&self, other: &Vec3) -> Vec3{
        Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }

    //Our function to reset our floats to be within range as we convert (has to be in u8 format for rgb to work)
    pub fn to_rgb(&self) -> [u8; 3]{
        fn f(num: f64) -> u8 {
            if num < 0.0{ 
                0
            }
            else if num >= 1.0{
                255
            }
            else {
                (num * 255.99) as u8
            }
        }
        [f(self.x), f(self.y), f(self.z)]
    }    

    pub fn magnitude(&self) -> f64{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Create a unit vector by normalizing the vector
    pub fn unit_vector(&self) -> Vec3{
        let mag = self.magnitude();
        if mag == 0.0 {
            Vec3{x: 0.0, y: 0.0, z: 0.0}
        } else {
            Vec3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            
            }
        }
    }

    pub fn dot(self, other: Vec3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

//A way of overloading to terminal
impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ 
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
    
}

impl Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3{
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3{
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

//Implement multiplication of Vec3 by a scalar f64
impl Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3{
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

//Implement multiplication of a scalar f64 by Vec3
impl Mul<Vec3> for f64{
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3{
        Vec3{
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3{
        if scalar == 0.0{
            panic!("Division by zero is not allowed");
        }
        Vec3{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}