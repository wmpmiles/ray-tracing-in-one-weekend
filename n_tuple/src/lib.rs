#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NTuple<T, const N: usize>([T; N])
where
    T: Copy + Clone + PartialEq + Default;

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

impl<T, const N: usize> std::ops::Index<usize> for NTuple<T, N>
where
    T: Copy + Clone + PartialEq + Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> std::convert::From<[T; N]> for NTuple<T, N>
where
    T: Copy + Clone + PartialEq + Default,
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
        assert_eq!(t1.map(|x| x + 1), t2)
    }

    #[test]
    fn combine_tuples() {
        let t1 = ntuple!(0, 1, 1, 2, 3, 5);
        let t2 = ntuple!(5, 3, 2, 1, 1, 0);
        let t3 = ntuple!(5, 4, 3, 3, 4, 5);
        assert_eq!(t1.combine(t2, |x, y| x + y), t3);
    }
}

