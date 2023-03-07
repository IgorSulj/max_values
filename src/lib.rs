// This file declares the struct, as well as defines its high-level functions and imports other modules
use core::slice::Iter;
use arrayvec::IntoIter;
use arrayvec::ArrayVec;


/// Main struct for getting max values out of given
#[derive(Debug, Clone)]
pub struct MaxValues<T: Ord, const N: usize> {
    data: ArrayVec<T, N>
}

impl<T: Ord, const N: usize> MaxValues<T, N> {
    /// Creates new empty [`MaxValues`] data structure
    pub fn new() -> Self {
        MaxValues { data: ArrayVec::new() }
    }

    /// Consumes self and returns [`ArrayVec`] of the values. Note, that returned array may contain less than N values, if less than N values are pushed to TopValues. 
    pub fn to_values(self) -> ArrayVec<T, N> {
        self.data
    }

    /// Returns immutable reference to the [`ArrayVec`] of values. Note, that returned array may contain less than N values, if less than N values are pushed to TopValues.
    /// The reason why there is no mutable version of this method is that mutating the elements of array can invalidate the binary heap used by this data structure.
    pub fn as_values(&self) -> &ArrayVec<T, N> {
        &self.data
    }

    /// Returns an iterator over the values of self.
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }
}

impl<T: Ord, const N: usize> Default for MaxValues<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord, const N: usize> AsRef<[T]> for MaxValues<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T: Ord, const N: usize> FromIterator<T> for MaxValues<T, N> {
    fn from_iter<IterT: IntoIterator<Item = T>>(iter: IterT) -> Self {
        let mut values = MaxValues::new();
        iter.into_iter().for_each(|x| values.push(x));
        return values;
    }
}

impl<T: Ord, const N: usize> IntoIterator for MaxValues<T, N> {
    type IntoIter = IntoIter<T, N>;
    type Item = T;

    fn into_iter(self) -> IntoIter<T, N> {
        self.data.into_iter()
    }
}

mod push;
mod iter_ext;
pub use iter_ext::MaxValuesIterExt;

#[cfg(test)]
mod tests {
    use crate::{MaxValues, MaxValuesIterExt};
    use std::collections::HashSet;

    #[test]
    fn test_max_values() {
        let mut values = MaxValues::<i32, 3>::new();
        values.push(2);
        assert_eq!(values.as_ref(), [2]);

        values.push(3);
        values.push(4);
        assert_eq!(values.iter().copied().collect::<HashSet<_>>(), HashSet::from([2, 3, 4]));

        values.push(1);
        assert_eq!(values.iter().copied().collect::<HashSet<_>>(), HashSet::from([2, 3, 4]));

        values.push(5);
        assert_eq!(values.iter().copied().collect::<HashSet<_>>(), HashSet::from([3, 4, 5]));

        values.push(4);
        assert_eq!(values.iter().copied().collect::<HashSet<_>>(), HashSet::from([4, 4, 5]));
    }

    #[test]
    fn test_iterator() {
        let values = [1, 5, 2, 4, 7, 10, 0, 15, 3];
        assert_eq!(values.into_iter().max_values::<3>().collect::<HashSet<_>>(), HashSet::from([7, 10, 15]));
    }
}
