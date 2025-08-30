#[derive(Debug)]
struct Sphere {}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_spheres_are_not_equal() {
        let s1 = Sphere {};
        let s2 = Sphere {};
        assert_ne!(s1, s2);
    }

    #[test]
    fn same_sphere_is_equal() {
        let s1 = Sphere {};
        assert_eq!(s1, s1);
    }
}
