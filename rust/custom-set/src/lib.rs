use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: Debug + Clone + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut items = input.to_vec();
        items.sort();
        items.dedup();

        CustomSet { items }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.as_slice().contains(element)
    }

    pub fn add(&mut self, element: T) {
        if self.contains(&element) {
            return;
        }

        self.items.push(element);
        self.items.sort()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        let other_slice = other.items.as_slice();

        self.items.iter().all(|item| other_slice.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        let other_slice = other.items.as_slice();

        !self.items.iter().any(|item| other_slice.contains(item))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let other_slice = other.items.as_slice();
        let items = self.items.iter()
            .cloned()
            .filter(|item| other_slice.contains(&item))
            .collect::<Vec<_>>();

        Self::new(&items)
    }

    pub fn difference(&self, other: &Self) -> Self {
        let other_slice = other.items.as_slice();
        let items = self.items.iter()
            .cloned()
            .filter(|item| !other_slice.contains(&item))
            .collect::<Vec<_>>();

        Self::new(&items)
    }

    pub fn union(&self, other: &Self) -> Self {
        let items = self.items.iter()
            .chain(other.items.iter())
            .cloned()
            .collect::<Vec<_>>();

        Self::new(&items)
    }
}
