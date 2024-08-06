use crate::ray::Ray;
use crate::dspace::*;

pub struct HitRecord{
    point: Vec3,
    normal: Vec3,
    trajectory: f64
}


/*impl HitRecord{
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord{

    }
}*/