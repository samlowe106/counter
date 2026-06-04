use std::ops;

mod counter {

    struct SignedCounter<T: Eq + std::hash::Hash> {
        counts: HashMap<T, isize>,
    }

    impl<T: Eq + std::hash::Hash> ops::Neg for Counter<T> {
        type Output = Counter<T>;

        fn neg(self) -> Counter<T> {
            Counter {
                counts: self.counts.into_iter().map(|(k, v)| (k, -v)).collect(),
            }
        }
    }
}
