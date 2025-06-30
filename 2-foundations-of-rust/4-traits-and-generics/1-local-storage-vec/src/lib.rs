use std::ops::{Index, Range, RangeFrom, RangeTo};
/// A growable, generic list that resides on the stack if it's small,
/// but is moved to the heap to grow larger if needed.
/// This list is generic over the items it contains as well as the
/// size of its buffer if it's on the stack.
#[derive(Debug)]
pub enum LocalStorageVec<T, const N: usize> {
    // TODO add some variants containing data
    // to make the compiler happy
    Stack { buf: [T; N], len: usize },
    Heap(Vec<T>),
}

// **Below `From` implementation is used in the tests and are therefore given. However,
// you should have a thorough look at it as they contain various new concepts.**
// This implementation is generic not only over the type `T`, but also over the
// constants `N` and 'M', allowing us to support conversions from arrays of any
// length to `LocalStorageVec`s of with any stack buffer size.
// In Rust, we call this feature 'const generics'
impl<T, const N: usize, const M: usize> From<[T; N]> for LocalStorageVec<T, M>
where
    // We require that `T` implement `Default`, in case we need to fill up our
    // stack-based array without resorting to uninitialized memory. Once
    // we are more proficient in working with uninitialized memory, we'll be
    // able to remove this bound.
    T: Default,
{
    fn from(array: [T; N]) -> Self {
        if N <= M {
            // In this case, the passed array should fit on the stack.

            // We crate an `Iterator` of the passed array,
            let mut it = array.into_iter();
            Self::Stack {
                // This is a trick for copying an array into another one that's
                // at least as long as the original, without having to create
                // default values more than strictly necessary. The `[(); M]`
                // array is zero-sized, meaning there's no cost to instantiate it.
                // The `map` call iterates over each of its items, and maps them to
                // the next item from the `array` passed to this function. If there
                // are no more items left from `array`, we insert the default specified
                // for `T`
                buf: [(); M].map(|_| it.next().unwrap_or_default()),
                // The length of the buffer on stack is the length of the original `array`: `N`
                len: N,
            }
        } else {
            // If the passed array does not fit, we'll resort to moving it to the heap instead
            Self::Heap(Vec::from(array))
        }
    }
}

impl<T, const N: usize> AsRef<[T]> for LocalStorageVec<T, N> {
    fn as_ref(&self) -> &[T] {
        match self {
            LocalStorageVec::Stack { buf, len } => &buf[..*len],
            LocalStorageVec::Heap(v) => v.as_ref(),
        }
    }
}

impl<T, const N: usize> AsMut<[T]> for LocalStorageVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        match self {
            LocalStorageVec::Stack { buf, len } => buf[..*len].as_mut(),
            LocalStorageVec::Heap(v) => v.as_mut(),
        }
    }
}

/// Create a LocalStorageVec from a Vec.
impl<T, const N: usize> From<Vec<T>> for LocalStorageVec<T, N> {
    fn from(v: Vec<T>) -> Self {
        LocalStorageVec::Heap(v)
    }
}

#[allow(unused)] // Silence warnings since there's no main function calling the methods.
impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Copy + Default,
{
    fn new() -> Self {
        LocalStorageVec::Stack {
            buf: [T::default(); N],
            len: 0,
        }
    }
}

#[allow(unused)] // Silence warnings since there's no main function calling the methods.
impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Default,
{
    fn clear(&mut self) {
        match self {
            LocalStorageVec::Stack { buf: _, len } => *len = 0,
            LocalStorageVec::Heap(v) => v.clear(),
        }
    }
}

#[allow(unused)] // Silence warnings since there's no main function calling the methods.
impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Clone,
{
    /// Adds the item to the end of the LSV.
    fn push(&mut self, item: T) {
        match self {
            LocalStorageVec::Stack { buf, len } => {
                if *len < N {
                    buf[*len] = item;
                    *len += 1;
                } else {
                    let mut v: Vec<T> = Vec::with_capacity(N + 1);
                    v.extend_from_slice(buf);
                    v.push(item);
                    *self = Self::Heap(v);
                }
            }
            LocalStorageVec::Heap(v) => v.push(item),
        }
    }

    /// Removes the last item and returns it.
    fn pop(&mut self) -> Option<T> {
        match self {
            LocalStorageVec::Stack { buf, len } => {
                if *len == 0 {
                    return None;
                } else {
                    *len -= 1;
                    Some(buf[*len].clone())
                }
            }
            LocalStorageVec::Heap(v) => {
                if v.is_empty() {
                    return None;
                } else {
                    v.pop()
                }
            }
        }
    }
}

#[allow(unused)] // Silence warnings since there's no main function calling the methods.
impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Copy,
{
    /// Inserts item into index, shifting values over to one higher index.
    fn insert(&mut self, index: usize, element: T) {
        match self {
            LocalStorageVec::Stack { buf, len } => {
                if *len >= N {
                    // Switch to a Vec
                    let mut v: Vec<T> = Vec::with_capacity(N + 1);
                    v.extend_from_slice(&buf[..index]);
                    v.push(element);
                    v.extend_from_slice(&buf[index..]);
                    *self = Self::Heap(v);
                } else {
                    // Shift index.. over to index+1..
                    buf.copy_within(index..*len, index + 1);
                    *len += 1;
                    buf[index] = element;
                }
            }
            LocalStorageVec::Heap(v) => {
                v.insert(index, element);
            }
        }
    }

    /// Removes an indexed item and returns it.
    fn remove(&mut self, index: usize) -> T {
        match self {
            LocalStorageVec::Stack { buf, len } => {
                if index == *len - 1 {
                    self.pop().unwrap()
                } else {
                    let removed = buf[index];
                    buf.copy_within((index + 1)..*len, index);
                    *len -= 1;
                    removed
                }
            }
            LocalStorageVec::Heap(v) => v.remove(index),
        }
    }
}

impl<T, const N: usize> Index<usize> for LocalStorageVec<T, N> {
    type Output = T; // Associated type.

    fn index(&self, idx: usize) -> &Self::Output {
        // match self {
        //     LocalStorageVec::Stack { buf, len: _ } => &buf[idx],
        //     LocalStorageVec::Heap(v) => &v[idx],
        // }
        &self.as_ref()[idx]
    }
}

impl<T, const N: usize> Index<RangeFrom<usize>> for LocalStorageVec<T, N> {
    type Output = [T]; // Associated type.

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.as_ref()[range]
    }
}

impl<T, const N: usize> Index<RangeTo<usize>> for LocalStorageVec<T, N> {
    type Output = [T]; // Associated type.

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.as_ref()[range]
    }
}

impl<T, const N: usize> Index<Range<usize>> for LocalStorageVec<T, N> {
    type Output = [T]; // Associated type.

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.as_ref()[range]
    }
}

#[allow(unused)]
impl<T, const N: usize> LocalStorageVec<T, N> {
    fn len(&self) -> usize {
        match self {
            LocalStorageVec::Stack { buf: _, len } => *len,
            LocalStorageVec::Heap(v) => v.len(),
        }
    }

    /// Consumes self and returns an iterator.
    fn into_iter(self) -> LocalStorageVecIter<T, N> {
        LocalStorageVecIter::new(self)
    }
}

/// Iterable struct for the LSV.
pub struct LocalStorageVecIter<T, const N: usize> {
    vec: LocalStorageVec<T, N>,
    counter: usize,
}

impl<T, const N: usize> LocalStorageVecIter<T, N> {
    fn new(vec: LocalStorageVec<T, N>) -> Self {
        Self { vec, counter: 0 }
    }
}

impl<T, const N: usize> Iterator for LocalStorageVecIter<T, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.vec.len() {
            return None;
        }

        let ret = self.vec.as_ref()[self.counter]; // Here's why it needs the Copy trait.
        self.counter += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.counter, Some(self.vec.len()))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.counter + n >= self.vec.len() {
            self.counter = self.vec.len();
            return None;
        }
        self.counter += n;
        Some(self.vec.as_ref()[self.counter])
    }

    // fn last(&mut self) -> Option<Self::Item> {
    //     if self.counter == self.vec.len() {
    //         return None;
    //     }
    //     self.counter = self.vec.len();
    //     Some(self.vec.as_ref()[self.vec.len() - 1])
    // }
}

#[cfg(test)]
mod test {
    use crate::LocalStorageVec;

    #[test]
    // Don't remove the #[ignore] attribute or your tests will take forever!
    #[ignore = "This test is just to validate the definition of `LocalStorageVec`. If it compiles, all is OK"]
    #[allow(unreachable_code, unused_variables)]
    fn it_compiles() {
        // Here's a trick to 'initialize' a type while not actually
        // creating a value: an infinite `loop` expression diverges
        // and evaluates to the 'never type' `!`, which, as is can never
        // actually be instantiated, coerces to any other type.
        // Some other ways of diverging are by calling the `panic!` or the `todo!`
        // macros.
        // More info:
        // - https://doc.rust-lang.org/rust-by-example/fn/diverging.html
        // - https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops
        let vec: LocalStorageVec<u32, 10> = loop {};
        match vec {
            LocalStorageVec::Stack { buf, len } => {
                let _buf: [u32; 10] = buf;
                let _len: usize = len;
            }
            LocalStorageVec::Heap(v) => {
                let _v: Vec<u32> = v;
            }
        }
    }

    // Uncomment me for part B
    #[test]
    fn it_from_vecs() {
        // The `vec!` macro creates a `Vec<T>` in a way that resembles
        // array-initialization syntax.
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::from(vec![1, 2, 3]);
        // Assert that the call to `from` indeed yields a `Heap` variant
        assert!(matches!(vec, LocalStorageVec::Heap(_)));

        let vec: LocalStorageVec<usize, 2> = LocalStorageVec::from(vec![1, 2, 3]);

        assert!(matches!(vec, LocalStorageVec::Heap(_)));
    }

    // Uncomment me for part C
    #[test]
    fn it_as_refs() {
        let vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);
        let vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);

        let mut vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice_mut: &mut [i32] = vec.as_mut();
        assert_eq!(slice_mut.len(), 128);
        let mut vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice_mut: &mut [i32] = vec.as_mut();
        assert_eq!(slice_mut.len(), 128);
    }

    // Uncomment me for part D
    #[test]
    fn it_constructs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::new();
        // Assert that the call to `new` indeed yields a `Stack` variant with zero length
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 0 }));
    }

    // Uncomment me for part D
    #[test]
    fn it_lens() {
        let vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
        let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
    }

    // Uncomment me for part D
    #[test]
    fn it_pushes() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
        for value in 0..128 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256))
    }

    // Uncomment me for part D
    #[test]
    fn it_pops() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn pops_in_order() {
        let mut vec: LocalStorageVec<i32, 128> = LocalStorageVec::new();
        for i in 0..128 {
            vec.push(i * 10);
        }
        println!();
        for i in (0..128).rev() {
            assert_eq!(vec.pop(), Some(i * 10));
        }
        assert_eq!(vec.pop(), None);
    }

    /// Andy Balaam's test.
    #[test]
    fn can_view_stack_lsv_as_ref_to_slice() {
        // Given a stack-based LocalStorageVec
        let vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);

        // When I view it as a slice
        let slice: &[i32] = vec.as_ref();

        // Then its contents are in the slice
        assert_eq!(slice, &[0, 1, 2]);
    }

    /// Andy Balaam's test.
    #[test]
    fn can_view_heap_lsv_as_ref_to_slice() {
        // Given a stack-based LocalStorageVec
        let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2, 3, 4]);

        // When I view it as a slice
        let slice: &[i32] = vec.as_ref();

        // Then its contents are in the slice
        assert_eq!(slice, &[0, 1, 2, 3, 4]);
    }

    // Uncomment me for part D
    #[test]
    fn it_inserts() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        vec.insert(1, 3);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 3, 1, 2],
                len: 4
            }
        ));

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        // assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3]);

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3, 4]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        // assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3, 4])
    }

    // Uncomment me for part D
    #[test]
    fn it_removes() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        dbg!(&vec);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 2, _, _],
                len: 2
            }
        ));
        assert_eq!(elem, 1);

        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        assert!(matches!(vec, LocalStorageVec::Heap(..)));
        assert_eq!(vec.as_ref(), &[0, 2]);
        assert_eq!(elem, 1);
    }

    #[test]
    #[should_panic]
    fn remove_out_of_range_heap_panics() {
        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.remove(5);
    }

    #[test]
    #[should_panic]
    fn remove_out_of_buffer_stack_panics() {
        let mut vec: LocalStorageVec<_, 8> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.remove(5);
    }

    #[test]
    fn remove_last_element_heap() {
        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2, 3]);
        let elem = vec.remove(3);
        assert_eq!(elem, 3);
    }

    #[test]
    fn remove_last_element_stack() {
        let mut vec: LocalStorageVec<_, 8> = LocalStorageVec::from([0, 1, 2, 3]);
        let elem = vec.remove(3);
        assert_eq!(elem, 3);
    }

    // Uncomment me for part D
    #[test]
    fn it_clears() {
        let mut vec: LocalStorageVec<_, 10> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 4 }));
        vec.clear();
        assert_eq!(vec.len(), 0);

        let mut vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn clear_push_and_pop() {
        let mut vec: LocalStorageVec<_, 10> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 4 }));
        vec.clear();
        assert_eq!(vec.len(), 0);

        vec.push(7);
        assert_eq!(vec.len(), 1);
        let elem = vec.remove(0);
        assert_eq!(elem, 7);
    }

    // Uncomment me for part E
    #[test]
    fn it_iters() {
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 32]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);

        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 128]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_size_hint() {
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 32]);
        let mut iter = vec.into_iter();
        iter.next();
        iter.next();
        let hint = iter.size_hint();
        assert_eq!(hint.0, 2);
        assert_eq!(hint.1, Some(32));
    }

    // Uncomment me for part F
    #[test]
    fn it_indexes() {
        let vec: LocalStorageVec<i32, 10> = LocalStorageVec::from([0, 1, 2, 3, 4, 5]);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[..2], [0, 1]);
        assert_eq!(vec[4..], [4, 5]);
        assert_eq!(vec[1..3], [1, 2]);
    }

    // Uncomment me for part H
    // #[test]
    // fn it_borrowing_iters() {
    //     let vec: LocalStorageVec<String, 10> = LocalStorageVec::from([
    //         "0".to_owned(),
    //         "1".to_owned(),
    //         "2".to_owned(),
    //         "3".to_owned(),
    //         "4".to_owned(),
    //         "5".to_owned(),
    //     ]);
    //     let iter = vec.iter();
    //     for _ in iter {}
    //     // This requires the `vec` not to be consumed by the call to `iter()`
    //     drop(vec);
    // }

    // Uncomment me for part J
    // #[test]
    // fn it_derefs() {
    //     use std::ops::{Deref, DerefMut};
    //     let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
    //     // `chunks` is a method that's defined for slices `[T]`, that we can use thanks to `Deref`
    //     let chunks = vec.chunks(4);
    //     let slice: &[_] = vec.deref();
    //
    //     let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
    //     let chunks = vec.chunks_mut(4);
    //     let slice: &mut [_] = vec.deref_mut();
    // }
}
