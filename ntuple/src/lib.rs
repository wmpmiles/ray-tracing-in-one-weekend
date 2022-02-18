use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NTuple<T, const N: usize>([T; N]);

pub trait NTupleNewtype<T, const N: usize> {
    fn ntuple(&self) -> NTuple<T, N>;
}

#[macro_export]
macro_rules! ntuple {
    ($($element:expr),*) => { NTuple::from([$($element, )*]) }
}

impl<T, const N: usize> NTuple<T, N>
where
    T: Default + Copy + Clone,
{
    pub fn map<F, U>(self, f: F) -> NTuple<U, N>
    where
        F: Fn(T) -> U,
        U: Copy + Clone,
    {
        NTuple(self.0.map(f))
    }

    pub fn combine<F, U>(self, rhs: Self, f: F) -> NTuple<U, N>
    where
        F: Fn(T, T) -> U,
        U: Default + Copy + Clone,
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

    pub fn permute(self, perms: [usize; N]) -> NTuple<T,N> {
        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = self[perms[i]];
        }
        NTuple::from(arr)
    }
}

impl<T, const N: usize> Default for NTuple<T, N>
where
    T: Copy + Clone + Default,
{
    fn default() -> Self {
        NTuple([T::default(); N])
    }
}

impl<T, const N: usize> std::ops::Index<usize> for NTuple<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> std::convert::From<[T; N]> for NTuple<T, N> {
    fn from(array: [T; N]) -> Self {
        NTuple(array)
    }
}

impl<T, const N: usize> std::convert::From<&[T]> for NTuple<T, N>
where
    T: Copy + Clone + std::fmt::Debug,
{
    fn from(slice: &[T]) -> Self {
        let array: [T; N] = slice.try_into().unwrap();
        NTuple::from(array)
    }
}

impl<T, const N: usize> std::convert::From<Vec<T>> for NTuple<T, N>
where
    T: Copy + Clone + std::fmt::Debug,
{
    fn from(vec: Vec<T>) -> Self {
        NTuple::from(&vec[..])
    }
}

impl<T, const N: usize> Serialize for NTuple<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for element in &self.0 {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for NTuple<T, N>
where
    T: Deserialize<'de> + Copy + Clone + std::fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<NTuple<T, N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seq: Vec<T> = Deserialize::deserialize(deserializer)?;
        assert!(
            seq.len() == N,
            "Cannot convert {:?} into NTuple of length {N}",
            &seq
        );
        Ok(NTuple::from(seq))
    }
}
