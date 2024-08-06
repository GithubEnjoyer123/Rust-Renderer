use crate::dspace::*;
use crate::ray::*;
//use crate::hit::*;

pub struct Sphere{
    pub center: Vec3,
    radius: f64,
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{
            center: center,
            radius: radius
        }
    }

    pub fn hit(&self, ray: &Ray) -> f64{
        let oc: Vec3 = ray.orig - self.center;
        let a: f64 = ray.dir.dot(ray.dir);
        let b: f64 = 2.0 * oc.dot(ray.dir);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b*b - 4.0*a*c;

        if discriminant < 0.0{
            return -1.0
        }else{
            return(b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
