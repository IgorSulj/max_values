use crate::MaxValues;

/// Iterator extension trait, 
/// which adds [`MaxValuesIterExt::max_values`] method
pub trait MaxValuesIterExt: Iterator {
    /// Returns iterator, which iterates over n biggest elements
    /// of given iterator.
    fn max_values<const N: usize>(self) -> arrayvec::IntoIter<Self::Item, N> where Self: Sized, Self::Item: Ord {
        MaxValues::from_iter(self).into_iter()
    }
}

impl <I: Iterator> MaxValuesIterExt for I {}
