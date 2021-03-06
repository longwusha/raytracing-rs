use hittable::{Hittable, HitRecord};
use ray::Ray;
use vec3::Vec3;
use vec3;
use material::{Material};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        
        let oc = ray.origin() - self.center;

        let a = vec3::dot(&ray.dir(), &ray.dir());
        let b = vec3::dot(&oc, &ray.dir());
        let c = vec3::dot(&oc, &oc) - self.radius * self.radius;
        
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material);

                return true;
            }
            let temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material);
                
                return true;
            }
        }
        
        false
    }
}