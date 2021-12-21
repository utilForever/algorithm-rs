/// To use SegTree, following condition has to be met.
///
/// - `func(func(a, b), c) == func(a, func(b, c))`
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
    pub fn new(size: usize, default: T, func: F) -> SegTree<T, F> {
        SegTree {
            container: vec![default; size << 2],
            func,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.container.len()
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
            self.func(
                self._get(node << 1, left, mid, start, end),
                self._get((node << 1) | 1, mid, right, start, end),
            )
        }
    }

    #[inline]
    pub fn get(&self, start: usize, end: usize) -> T {
        debug_assert!(start < end, "start = {} > end = {}", start, end);
        debug_assert!(start >= 0, "start = {} < 0", start);
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

        self.container[node] = self.func(a, b);
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: T) {
        debug_assert!(
            0 <= index && index < self.len(),
            "index = {} is out of bounds",
            index
        );

        self._set(1, 0, self.len(), index, value);
    }
}

impl<T, F> std::ops::Index<usize> for SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index, index)
    }
}

impl<T, F> std::ops::Index<(usize, usize)> for SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<T, F> std::ops::Index<std::ops::Range<usize>> for SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    type Output = T;

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        self.get(index.start, index.end)
    }
}
