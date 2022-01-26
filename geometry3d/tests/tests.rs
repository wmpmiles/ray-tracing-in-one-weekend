#[cfg(test)]
mod vec3_tests {
    use geometry3d::*;

    #[test]
    fn cannonical_basis_vectors() {
        assert_eq!(Vec3::e0().x(), 1.0);
        assert_eq!(Vec3::e0().y(), 0.0);
        assert_eq!(Vec3::e0().z(), 0.0);

        assert_eq!(Vec3::e1().x(), 0.0);
        assert_eq!(Vec3::e1().y(), 1.0);
        assert_eq!(Vec3::e1().z(), 0.0);

        assert_eq!(Vec3::e2().x(), 0.0);
        assert_eq!(Vec3::e2().y(), 0.0);
        assert_eq!(Vec3::e2().z(), 1.0);
    }

    #[test]
    fn new_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn vector_quadrance() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.quadrance(), 9.0);
    }

    #[test]
    fn vector_length() {
        let v = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(v.length(), 7.0);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::new(1.0, 2.0, 2.0).unit().unwrap();
        assert_eq!(v.x(), 1.0 / 3.0);
        assert_eq!(v.y(), 2.0 / 3.0);
        assert_eq!(v.z(), 2.0 / 3.0);
        let n = Vec3::new(0.0, 0.0, 0.0).unit();
        assert_eq!(n, None);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(v1.dot(v2), 1.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(Vec3::e0().cross(Vec3::e1()), Vec3::e2());
    }

    #[test]
    fn add_vectors() {
        let v1 = Vec3::e0() + Vec3::e1() + Vec3::e2();
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = -Vec3::e0() - Vec3::e1() - Vec3::e2();
        let v2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn negate_vector() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(-v1, v2);
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(3.0 * v1, v2);
    }

    #[test]
    fn scalar_division() {
        let v1 = Vec3::new(5.0, 5.0, 5.0);
        let v2 = Vec3::new(2.5, 2.5, 2.5);
        assert_eq!(v1 / 2.0, v2);
    }

    #[test]
    fn vector_projection() {
        let v1 = Vec3::e0() + Vec3::e1();
        let v2 = 2.0 * Vec3::e0();
        assert_eq!(v1.projection(v2), Vec3::e0());
    }

    #[test]
    fn position_vector() {
        let p = Point3::new(0.1, 0.2, 0.3);
        let v: Vec3 = p.into();
        assert_eq!(v.x(), 0.1);
        assert_eq!(v.y(), 0.2);
        assert_eq!(v.z(), 0.3);
    }
}

#[cfg(test)]
mod point3_tests {
    use geometry3d::*;

    #[test]
    fn new_point() {
        let p = Point3::new(22.0, 33.0, 44.0);
        assert_eq!(p.x(), 22.0);
        assert_eq!(p.y(), 33.0);
        assert_eq!(p.z(), 44.0);
    }

    #[test]
    fn from_vector() {
        let v = Vec3::new(-5.0, -8.0, -13.0);
        let p: Point3 = v.into();
        assert_eq!(p.x(), -5.0);
        assert_eq!(p.y(), -8.0);
        assert_eq!(p.z(), -13.0);
    }

    #[test]
    fn add_vector_to_point() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(0.1, 0.4, 0.9);
        let p2 = p1 + v;
        assert_eq!(p2, v.into());
    }

    #[test]
    fn sub_vector_from_point() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(0.1, 0.4, 0.9);
        let p2 = p1 - v;
        assert_eq!(p2, (-v).into());
    }

    #[test]
    fn difference_of_two_ponits() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(4.0, 8.0, 16.0);
        assert_eq!(p2 - p1, Vec3::from(p2));
    }
}

#[cfg(test)]
mod ray3_tests {
    use geometry3d::*;

    #[test]
    fn create_and_access() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::e0();
        let time = 0.0;
        let ray = Ray3 {
            origin,
            direction,
            time,
        };
        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
        assert_eq!(time, ray.time);
    }

    #[test]
    fn at() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::e0();
        let time = 0.0;
        let ray = Ray3 {
            origin,
            direction,
            time,
        };

        let p1 = origin + direction;
        let p2 = origin + 2.0 * direction;
        assert_eq!(ray.at(1.0), p1);
        assert_eq!(ray.at(2.0), p2);
    }
}

#[cfg(test)]
mod aabb_tests {
    use geometry3d::*;

    #[test]
    fn create() {
        let aabb = AABB::new(Point3::new(1.0, 1.0, 1.0), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.lo(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.hi(), Point3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn hit() {
        let aabb = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));

        let mut r1 = Ray3 {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vec3::e0(),
            time: 0.0,
        };

        assert_eq!(aabb.hit(r1, 0.0, 1.0), false);
        assert_eq!(aabb.hit(r1, 0.1, 0.9), false);
        assert_eq!(aabb.hit(r1, 1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 0.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, -0.5), false);
        assert_eq!(aabb.hit(r1, 1.5, 2.0), false);

        r1.direction = -Vec3::e0();
        assert_eq!(aabb.hit(r1, 0.0, 1.0), false);
        assert_eq!(aabb.hit(r1, 0.1, 0.9), false);
        assert_eq!(aabb.hit(r1, 1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 0.0), false);
        assert_eq!(aabb.hit(r1, -2.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, -0.5), false);
        assert_eq!(aabb.hit(r1, -2.0, -1.5), false);

        r1.direction = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(aabb.hit(r1, 0.0, 1.0), true);
        assert_eq!(aabb.hit(r1, 0.1, 0.9), true);
        assert_eq!(aabb.hit(r1, 1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 0.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 2.0), true);
        assert_eq!(aabb.hit(r1, -1.0, -0.5), false);
        assert_eq!(aabb.hit(r1, 1.5, 2.0), false);

        r1.direction = -r1.direction;
        assert_eq!(aabb.hit(r1, -1.0, 0.0), true);
        assert_eq!(aabb.hit(r1, -0.9, -0.1), true);
        assert_eq!(aabb.hit(r1, -2.0, -1.0), false);
        assert_eq!(aabb.hit(r1, 0.0, 1.0), false);
        assert_eq!(aabb.hit(r1, -2.0, 1.0), true);
        assert_eq!(aabb.hit(r1, 0.5, 1.0), false);
        assert_eq!(aabb.hit(r1, -2.0, -1.5), false);

        r1.direction = -r1.direction;
        r1.origin = Point3::new(1.0, 0.0, 0.0);
        assert_eq!(aabb.hit(r1, 0.0, 1.0), false);
        assert_eq!(aabb.hit(r1, 0.1, 0.9), false);
        assert_eq!(aabb.hit(r1, 1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 0.0), false);
        assert_eq!(aabb.hit(r1, -1.0, 2.0), false);
        assert_eq!(aabb.hit(r1, -1.0, -0.5), false);
        assert_eq!(aabb.hit(r1, 1.5, 2.0), false);
    }

    #[test]
    fn merge() {
        let aabb1 = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
        let aabb2 = AABB::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));

        let merged = AABB::merge(Some(aabb1), Some(aabb2)).unwrap();
        assert_eq!(merged.lo(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(merged.hi(), Point3::new(2.0, 2.0, 2.0));

        let merged = AABB::merge(None, None);
        assert_eq!(merged.is_none(), true);

        let merged = AABB::merge(Some(aabb1), None).unwrap();
        assert_eq!(merged.lo(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(merged.hi(), Point3::new(1.0, 1.0, 1.0));

        let merged = AABB::merge(None, Some(aabb2)).unwrap();
        assert_eq!(merged.lo(), Point3::new(1.0, 1.0, 1.0));
        assert_eq!(merged.hi(), Point3::new(2.0, 2.0, 2.0));
    }
}
