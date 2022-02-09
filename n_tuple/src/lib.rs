use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_big_array::BigArray;

pub trait TupleMember: Copy + Clone + PartialEq + Default + Serialize + DeserializeOwned {}
impl<T: Copy + Clone + PartialEq + Default + Serialize + DeserializeOwned> TupleMember for T {}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct NTuple<T, const N: usize>(
    #[serde(with = "BigArray")]
    [T; N]
)
where
    T: TupleMember;

/* Behaviours:
 * - create n-tuple with list of values
 * - access individual tuple values
 * - apply a transformation to each value in the tuple
 * - combine tuples arbitrarily
 */

#[macro_export]
macro_rules! ntuple {
    ($($element:expr),*) => { NTuple::from([$($element, )*]) }
}

impl<T, const N: usize> NTuple<T, N> 
where
    T: TupleMember,
{
    pub fn map<F, U>(self, f: F) -> NTuple<U, N>
    where
        F: Fn(T) -> U,
        U: TupleMember,
    {
        NTuple(self.0.map(f))
    }

    pub fn combine<F, U>(self, rhs: Self, f: F) -> NTuple<U, N>
    where
        F: Fn(T, T) -> U,
        U: TupleMember,
    {
        let mut result: NTuple<U, N> = NTuple::default();
        for i in 0..N {
            result.0[i] = f(self[i], rhs[i]);
        }
        result
    }

    pub fn reduce<F>(self, f: F) -> T
    where
        F: Fn(T, T) -> T,
    {
        assert!(N > 0, "Cannot reduce the 0-tuple.");
        let mut acc = self[0];
        for i in 1..N {
            acc = f(acc, self[i]);
        }
        acc
    }

    pub fn fold<F>(self, start: T, f: F) -> T
    where
        F: Fn(T, T) -> T,
    {
        let mut acc = start;
        for i in 0..N {
            acc = f(acc, self[i]);
        }
        acc
    }
}

impl<T, const N: usize> Default for NTuple<T, N>
where
    T: TupleMember,
{
    fn default() -> Self {
        NTuple([T::default(); N])
    }
}

impl<T, const N: usize> std::ops::Index<usize> for NTuple<T, N>
where
    T: TupleMember,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> std::convert::From<[T; N]> for NTuple<T, N>
where
    T: TupleMember,
{
    fn from(array: [T; N]) -> Self {
        NTuple(array)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_access_ntuple() {
        let t1 = ntuple!(0, 1, 2, 3, 4, 5);
        let t2 = ntuple!(1, 2, 3, 4, 5, 6);
        assert_eq!(t1[0], 0);
        assert_eq!(t1[1], 1);
        assert_eq!(t1[2], 2);
        assert_eq!(t1[3], 3);
        assert_eq!(t1[4], 4);
        assert_eq!(t1[5], 5);
        assert_ne!(t1, t2);
    }

    #[test]
    fn transform_tuple() {
        let t1 = NTuple::from([0; 4]);
        let t2 = NTuple::from([1; 4]);
        assert_eq!(t1.map(|x| x + 1), t2);

        let t3 = NTuple::from([3; 3]);
        let t4 = NTuple::from([3.0; 3]);
        assert_eq!(t3.map(|x| x as f64), t4);
    }

    #[test]
    fn combine_tuples() {
        let t1 = ntuple!(0, 1, 1, 2, 3, 5);
        let t2 = ntuple!(5, 3, 2, 1, 1, 0);
        let t3 = ntuple!(5, 4, 3, 3, 4, 5);
        assert_eq!(t1.combine(t2, |x, y| x + y), t3);
    }

    #[test]
    fn fold_tuples() {
        let t1 = ntuple!(1, 2, 3);
        assert_eq!(t1.reduce(|acc, x| acc + x), 6);
        assert_eq!(t1.fold(10, |acc, x| acc + x), 16);
    }

    #[test]
    #[should_panic]
    fn reduce_zero_tuple() {
        let zero = NTuple::from([0; 0]);
        zero.reduce(|acc, x| acc + x);
    }

    #[test]
    fn de_serialize() {
        let t = ntuple!(1, 2, 3);
        let s = serde_json::to_string(&t).unwrap();
        let t_de: NTuple<i32, 3> = serde_json::from_str(&s).unwrap();
        assert_eq!(t_de, t);
    }
}

