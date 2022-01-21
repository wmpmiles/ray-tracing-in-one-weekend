pub mod vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NTuple<T, const N: usize>([T; N])
where
    T: Copy + Clone + PartialEq + Default;

#[macro_export]
macro_rules! ntuple {
    ($($element:expr),*) => { NTuple ( [$($element,)*] ) }
}

impl<T, const N: usize> NTuple<T, N> 
where
    T: Copy + Clone + PartialEq + Default,
{
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        NTuple(self.0.map(f))
    }

    pub fn combine<F>(self, rhs: Self, f: F) -> Self 
    where
        F: Fn(T, T) -> T,
    {
        let mut result = self.0;
        for i in 0..N {
            result[i] = f(self.0[i], rhs.0[i]);
        }
        NTuple(result)
    }
}

impl<T, const N: usize> Default for NTuple<T, N>
where
    T: Copy + Clone + PartialEq + Default,
{
    fn default() -> Self {
        NTuple([T::default(); N])
    }
}

