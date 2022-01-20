#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3<T>
where
    T: From<u32>,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

/* Behaviours:
 * - cannonical basis vectors
 * - create vec3 from x, y, z
 * - create vec3 from scalar s
 * - map
 * - calc quadrance
 * - calc length
 * - to unit vector
 * - dot product
 * - cross product
 * - projection
 * - reflection
 * - negate
 * - add
 * - subtract
 * - scalar multiplication
 * - scalar division
 */

impl<T: From<u32>> Vec3<T> {
    pub fn e0() -> Self {
        Vec3 {
            x: 1.into(),
            y: 0.into(),
            z: 0.into(),
        }
    }

    pub fn e1() -> Self {
        Vec3 {
            x: 0.into(),
            y: 1.into(),
            z: 0.into(),
        }
    }

    pub fn e2() -> Self {
        Vec3 {
            x: 0.into(),
            y: 0.into(),
            z: 1.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannonical_basis_vectors() {
        assert_eq!(Vec3::<u32>::e0().x, 1);
        assert_eq!(Vec3::<u32>::e0().y, 0);
        assert_eq!(Vec3::<u32>::e0().z, 0);

        assert_eq!(Vec3::<u32>::e1().x, 0);
        assert_eq!(Vec3::<u32>::e1().y, 1);
        assert_eq!(Vec3::<u32>::e1().z, 0);

        assert_eq!(Vec3::<u32>::e2().x, 0);
        assert_eq!(Vec3::<u32>::e2().y, 0);
        assert_eq!(Vec3::<u32>::e2().z, 1);
    }
}
