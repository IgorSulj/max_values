# MaxValues
### Package, that allows you to effectively get max values out of any sequence
For full documentation, see [this](docs.rs/max_values/1.0.0/max_values)

## Basic usage
The basic usage of this package looks like this
```rust
use max_values::MaxValues;

fn main() {
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
```
Beware, that ```MaxValues``` struct doesn't guarantee any order of elements. That's why we're transforming it into hashset for ```assert_eq``` macro.

## Using iterator adaptor
Common pattern is to iterate through collection and push it elements to `MaxValues` like this:
```rust
use max_values::MaxValues;

fn main() {
    let arr = [0, 1, 5, 7, 2, 3];

    let values = MaxValues::<i32, 3>::new();
    for i in arr {
        values.push(i);
    }

    assert_eq!(values.into_iter().collect::<HashSet<_>>, HashSet::from([3, 5, 7]));
}
```

That's why ```MaxValues``` implements ```FromIterable<T>```:
```rust
let arr = [0, 1, 5, 7, 2, 3];
let values = MaxValues::<i32, 3>::from_iter(arr.into_iter());
assert_eq!(values.into_iter().collect::<HashSet<_>>(), HashSet::from([3, 5, 7]));
```

Also, you can use iterator extension trait ```MaxValuesIterExt``` to iterate over max values of iterator:
```rust
use max_values::MaxValuesIterExt;

fn main() {
    let values = [1, 5, 2, 4, 7, 10, 0, 15, 3];
    assert_eq!(
        values.into_iter().max_values::<3>().collect::<HashSet<_>>(),
        HashSet::from([7, 10, 15])
    );
}
```
