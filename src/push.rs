// Separate module for the algorithm itself.
use crate::MaxValues;

impl<T: Ord, const N: usize> MaxValues<T, N> {
    /// Pushes an element into the data structure, 
    /// if it is bigger than one of the elements.
    /// May replace one of the previously pushed elements.
    pub fn push(&mut self, value: T) {
        if self.data.len() < N {
            self.push_back(value);
        } else {
            self.push_forward(value);
        }
    }

    // Push back to binary heap
    fn push_back(&mut self, value: T) {
        self.data.push(value);
        let mut index = self.data.len();
        while index > 1 && self.data[index / 2 - 1] > self.data[index - 1] {
            self.data.swap(index / 2 - 1, index - 1);
            index /= 2;
        }
    }

    // Push forward to binary heap. Also may leave the heap as it is if the value is too small.
    fn push_forward(&mut self, value: T) {
        if self.data[0] > value {
            return;
        }

        self.data[0] = value;
        let mut index = 1;
        while index * 2 < self.data.len() {
            let left = index * 2;
            let right = index * 2 + 1;
            let mut j = left;
            if right <= self.data.len() && self.data[left - 1] > self.data[right - 1] { 
                j = right;
            }
            if self.data[index - 1] > self.data[j - 1] {
                self.data.swap(index - 1, j - 1);
                index = j;
            } else {
                break;
            }
        }
    }
}
