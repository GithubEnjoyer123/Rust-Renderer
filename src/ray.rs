use crate::dspace::*;
use crate::sphere::*;

pub struct Ray{
    pub orig: Point3d,
    pub dir: Vec3,
}

impl Ray{
    pub fn new(origin: Point3d, direction: Vec3) -> Ray{
        Ray{ 
            orig: origin, 
            dir: direction 
        }
    }

    pub fn at(&self, distance: f64) -> Point3d{
        self.orig + distance * self.dir
    }

    pub fn color(&self) -> Color{
        let sphere = Sphere::new(Vec3::new(0.0,0.0,1.0), 0.5);
        let t = sphere.hit(self);
        if t > 0.0{
            let n:Vec3 = self.at(t) + Vec3::new(0.0,0.0,1.0);
            n.unit_vector();
            return 0.5 * Color::new(n.x+1.0, n.y+1.0, n.z+1.0);
        }
        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
    
}