use crate::{
    intersections::Intersection, materials::Material, matrix::Matrix, rays::Ray, sphere::Sphere,
    tuple::Tuple,
};

static mut ID_TRACKER: u64 = 0;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
}

#[derive(Clone, Debug)]
pub struct Shape {
    id: u64,
    pub shape_type: ShapeType,
    transform: Matrix,
    material: Material,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Shape {
        let id = unsafe {
            ID_TRACKER += 1;
            ID_TRACKER
        };
        Shape {
            id,
            shape_type,
            transform: Matrix::identity_4x4(),
            material: Material::default(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transformation: Matrix) {
        self.transform = transformation;
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if self.shape_type == ShapeType::Sphere {
            Sphere::intersect(&self, ray)
        } else {
            todo!()
        }
    }

    pub fn normal_at(&self, point: &Tuple) -> Tuple {
        if self.shape_type == ShapeType::Sphere {
            Sphere::normal_at(&self, point)
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        rays::ray,
        transformations::{rotation_z, scaling, translation},
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn default_transformation() {
        let s = Shape::new(ShapeType::Sphere);

        assert_eq!(s.transform(), &Matrix::identity_4x4())
    }

    #[test]
    fn assign_transformation() {
        let mut s = Shape::new(ShapeType::Sphere);

        s.set_transform(translation(2.0, 3.0, 4.0));

        assert_eq!(s.transform(), &translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s = Shape::new(ShapeType::Sphere);

        assert_eq!(s.material(), &Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = Shape::new(ShapeType::Sphere);
        let m = Material::default().ambient(1.0);

        s.set_material(m);

        assert_eq!(s.material(), &Material::default().ambient(1.0));
    }

    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(translation(5.0, 0.0, 0.0));
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(translation(0.0, 1.0, 0.0));

        let n = Shape::normal_at(&s, &point(0.0, 1.70711, -0.70711));

        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m);

        let n = Shape::normal_at(&s, &point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
