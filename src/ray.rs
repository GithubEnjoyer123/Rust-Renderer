use crate::dspace::*;

pub struct Ray{
    orig: Point3d,
    dir: Vec3,
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
        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}