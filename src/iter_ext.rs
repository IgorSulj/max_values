use crate::MaxValues;

pub trait MaxValuesIterExt: Iterator {
    fn max_values<const N: usize>(self) -> arrayvec::IntoIter<Self::Item, N> where Self: Sized, Self::Item: Ord {
        MaxValues::from_iter(self).into_iter()
    }
}

impl <I: Iterator> MaxValuesIterExt for I {}
