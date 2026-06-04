mod counter {

    use std::{cmp::Ordering, collections::HashMap, ops};

    struct Counter<T: Eq + std::hash::Hash> {
        counts: HashMap<T, usize>,
    }

    impl<T: Eq + std::hash::Hash> Counter<T> {
        pub fn new() -> Self {
            Self {
                counts: HashMap::new(),
            }
        }

        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut counter = Self::new();
            for item in iter {
                counter.add(item);
            }
            counter
        }

        pub fn add(&mut self, item: T) {
            *self.counts.entry(item).or_insert(0) += 1;
        }

        pub fn get(&self, item: &T) -> usize {
            self.counts.get(item).copied().unwrap_or(0)
        }

        pub fn mode(&self) -> Vec<(&T, usize)> {
            let mut entries: Vec<_> = self.counts.iter().map(|(k, &v)| (k, v)).collect();
            entries.sort_by(|a, b| b.1.cmp(&a.1));
            entries
        }

        pub fn sort_by(&self, ord: Ordering) {
            todo!()
        }

        pub fn len(&self) -> usize {
            self.counts.len()
        }

        pub fn remove(&self, item: &T) {
            todo!()
        }
    }

    impl<T: Eq + std::hash::Hash> ops::Index<&T> for Counter<T> {
        type Output = usize;

        fn index(&self, item: &T) -> &usize {
            self.counts.get(item).unwrap_or(&0)
        }
    }

    impl<T: Eq + std::hash::Hash> ops::AddAssign for Counter<T> {
        fn add_assign(&mut self, other: Counter<T>) {
            for (item, count) in other.counts {
                *self.counts.entry(item).or_insert(0) += count;
            }
        }
    }

    impl<T: Eq + std::hash::Hash> ops::Add for Counter<T> {
        type Output = Counter<T>;

        fn add(mut self, other: Counter<T>) -> Counter<T> {
            self += other;
            self
        }
    }

    impl<T: Eq + std::hash::Hash> ops::SubAssign for Counter<T> {
        fn sub_assign(&mut self, other: Counter<T>) {
            for (item, count) in other.counts {
                let entry = self.counts.entry(item).or_insert(0);
                *entry = entry.saturating_sub(count);
            }
        }
    }

    impl<T: Eq + std::hash::Hash> ops::Sub for Counter<T> {
        type Output = Counter<T>;

        fn sub(mut self, other: Counter<T>) -> Counter<T> {
            self -= other;
            self
        }
    }
}
