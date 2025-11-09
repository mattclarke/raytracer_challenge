use crate::{
    intersections::Intersection,
    materials::Material,
    matrix::{inverse, Matrix},
    plane::Plane,
    rays::{transform, Ray},
    sphere::Sphere,
    tuple::{normalise, Tuple},
};

static mut ID_TRACKER: u64 = 0;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    Plane,
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
        // Transform the ray by the inverse of the shape's transform.
        // This puts the shape at the origin which makes the maths simpler.
        let ray = transform(ray, &inverse(self.transform()));

        match self.shape_type {
            ShapeType::Sphere => Sphere::local_intersect(self, &ray),
            ShapeType::Plane => Plane::local_intersect(self, &ray),
        }
    }

    pub fn normal_at(&self, point: &Tuple) -> Tuple {
        // Convert the point to object space.
        let obj_point = inverse(self.transform()) * point;

        // Calculate the normal in object space.
        let local_normal = match self.shape_type {
            ShapeType::Sphere => Sphere::local_normal(self, &obj_point),
            ShapeType::Plane => Plane::local_normal(self, &obj_point),
        };

        // Transform back to world space.
        let mut world_normal = inverse(self.transform()).transpose() * local_normal;
        world_normal.w = 0.0;
        normalise(&world_normal)
    }
}

#[cfg(test)]
mod tests {
    use crate::transformations::translation;

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
}
