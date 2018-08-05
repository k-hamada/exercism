use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School {
    students: RefCell<BTreeMap<u32, BTreeSet<String>>>
}

impl School {
    pub fn new() -> School {
        School  { students: RefCell::new(BTreeMap::new()) }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.borrow_mut()
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.borrow()
            .keys()
            .cloned()
            .collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students.borrow()
            .get(&grade)
            .map(|grade| grade.iter().cloned().collect())
    }
}
