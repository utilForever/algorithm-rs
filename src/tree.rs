/// This is a segment tree that can be used widely.
/// Index is 0-based, range calculation is based on `std::ops::Range`.
///
/// # Conditions
///
/// - `func(func(a, b), c) == func(a, func(b, c))` (associated law)
///
/// If the conditions are not met, the behavior is undefined
///
/// # Examples
///
/// ```
/// use algorithm_rs::tree::SegTree;
///
/// let mut tree = SegTree::new(10, 0, |a, b| a + b);
/// tree.set(2, 3);
/// assert_eq!(tree.get(1, 3), 3);
/// ```
pub struct SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    container: Vec<T>,
    func: F,
}

impl<T, F> SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    /// Makes new `SegTree`
    pub fn new(size: usize, default: T, func: F) -> SegTree<T, F> {
        debug_assert!(size > 0, "SegTree cannot be empty");
        SegTree {
            container: vec![default; size << 2],
            func,
        }
    }

    fn apply_vec(&mut self, node: usize, left: usize, right: usize, vec: &Vec<T>) {
        if left + 1 == right {
            self.container[node] = vec[left];
            return;
        }

        let mid = (left + right) >> 1;
        self.apply_vec(node << 1, left, mid, vec);
        self.apply_vec((node << 1) + 1, mid, right, vec);

        let a = self.container[node << 1];
        let b = self.container[(node << 1) + 1];
        self.container[node] = (self.func)(a, b);
    }

    /// Makes new `SegTree` based on `vec`
    /// The indices would match with items.
    pub fn from_vec(vec: &Vec<T>, func: F) -> SegTree<T, F> {
        debug_assert!(vec.len() > 0, "SegTree cannot be empty");
        let mut tree = Self::new(vec.len(), vec[0], func);
        tree.apply_vec(1, 0, tree.len(), vec);
        tree
    }

    /// Returns length of the `SegTree`
    #[inline]
    pub fn len(&self) -> usize {
        self.container.len() >> 2
    }

    fn _get(&self, node: usize, left: usize, right: usize, start: usize, end: usize) -> T {
        if start <= left && right <= end {
            return self.container[node];
        }

        let mid = (left + right) >> 1;
        if end <= mid {
            self._get(node << 1, left, mid, start, end)
        } else if mid <= start {
            self._get((node << 1) | 1, mid, right, start, end)
        } else {
            (self.func)(
                self._get(node << 1, left, mid, start, end),
                self._get((node << 1) | 1, mid, right, start, end),
            )
        }
    }

    /// Calculates the functions in range.
    /// Acts like fold function within range.
    ///
    /// Indices are 0-based, `end` index is not included in calculation
    ///
    /// **Time Complexity**: `O(log n)` where `n = self.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithm_rs::tree::SegTree;
    ///
    /// let tree = SegTree::from_vec(&vec![1, 2, 3, 4], |a, b| a + b);
    /// assert_eq!(tree.get(1, 4), 9); // same as `2 + 3 + 4`
    /// ```
    #[inline]
    pub fn get(&self, start: usize, end: usize) -> T {
        debug_assert!(start < end, "start = {} > end = {}", start, end);
        debug_assert!(end <= self.len(), "end = {} > length = {}", end, self.len());

        self._get(1, 0, self.len(), start, end)
    }

    fn _set(&mut self, node: usize, left: usize, right: usize, index: usize, value: T) {
        if left + 1 == right {
            self.container[node] = value;
            return;
        }

        let mid = (left + right) >> 1;
        if index < mid {
            self._set(node << 1, left, mid, index, value);
        } else {
            self._set((node << 1) | 1, mid, right, index, value);
        }

        let a = self.container[node << 1];
        let b = self.container[(node << 1) | 1];

        self.container[node] = (self.func)(a, b);
    }

    /// Set the value of `SegTree`
    ///
    /// Index is 0-based
    ///
    /// **Time Complexity**: `O(log n)` where `n = self.len()`
    #[inline]
    pub fn set(&mut self, index: usize, value: T) {
        debug_assert!(
            index < self.len(),
            "index = {} >= len = {}",
            index,
            self.len()
        );

        self._set(1, 0, self.len(), index, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_vec_test() {
        let v = vec![1, 3, 2, 4];
        let tree = SegTree::from_vec(&v, |a, b| a + b);
        assert_eq!(tree.get(1, 3), 5);
    }

    #[test]
    fn length_test() {
        let tree = SegTree::new(10, 0, |a, b| a + b);
        assert_eq!(tree.len(), 10);
    }

    #[test]
    fn single_index_test() {
        let mut tree = SegTree::new(10, 0, |a, b| a + b);
        tree.set(1, 2);
        assert_eq!(tree.get(1, 2), 2);
    }

    #[test]
    fn range_index_test() {
        let mut tree = SegTree::new(10, 0, |a, b| a + b);
        tree.set(1, 2);
        tree.set(3, 4);
        assert_eq!(tree.get(1, 4), 6);
    }

    #[test]
    fn change_test() {
        let mut tree = SegTree::new(10, 0, |a, b| a + b);
        tree.set(3, 4);
        assert_eq!(tree.get(3, 4), 4);
        tree.set(3, 2);
        assert_eq!(tree.get(3, 4), 2);
    }
}
