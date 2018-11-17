use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();

    let (start, ignore) = match start_bucket {
        &Bucket::One => ((Buckets::new(capacity_1, 0)), Buckets::new(0, capacity_2)),
        &Bucket::Two => ((Buckets::new(0, capacity_2)), Buckets::new(capacity_1, 0)),
    };
    queue.push_back(start);
    checked.insert(ignore);

    while let Some(buckets) = queue.pop_front() {
        if checked.contains(&buckets) {
            continue;
        }

        if let Some(stats) = buckets.check(goal) {
            return stats;
        }

        queue.push_back(buckets.move_to_one(capacity_1));
        queue.push_back(buckets.move_to_two(capacity_2));
        queue.push_back(buckets.fill_at_one(capacity_1));
        queue.push_back(buckets.fill_at_two(capacity_2));
        queue.push_back(buckets.empty_at_one());
        queue.push_back(buckets.empty_at_two());

        checked.insert(buckets);
    }

    return BucketStats {
        moves: 0,
        goal_bucket: Bucket::One,
        other_bucket: 0,
    };
}

struct Buckets {
    bucket_1: u8,
    bucket_2: u8,
    moves: u8,
}

impl Buckets {
    fn new(bucket_1: u8, bucket_2: u8) -> Self {
        Buckets::new_with_moved(bucket_1, bucket_2, 1)
    }

    fn new_with_moved(bucket_1: u8, bucket_2: u8, moves: u8) -> Self {
        Buckets {
            bucket_1,
            bucket_2,
            moves,
        }
    }

    fn move_to_one(&self, capacity_1: u8) -> Self {
        let space_1 = (capacity_1 - self.bucket_1).min(self.bucket_2);
        Buckets::new_with_moved(
            self.bucket_1 + space_1,
            self.bucket_2 - space_1,
            self.moves + 1,
        )
    }

    fn move_to_two(&self, capacity_2: u8) -> Self {
        Buckets::move_to_one(&self.swap(), capacity_2).swap()
    }

    fn fill_at_one(&self, capacity_1: u8) -> Self {
        Buckets::new_with_moved(self.bucket_1.max(capacity_1), self.bucket_2, self.moves + 1)
    }

    fn fill_at_two(&self, capacity_2: u8) -> Self {
        Buckets::new_with_moved(self.bucket_1, self.bucket_2.max(capacity_2), self.moves + 1)
    }

    fn empty_at_one(&self) -> Self {
        Buckets::new_with_moved(0, self.bucket_2, self.moves + 1)
    }

    fn empty_at_two(&self) -> Self {
        Buckets::empty_at_one(&self.swap()).swap()
    }

    fn swap(&self) -> Self {
        Buckets::new_with_moved(self.bucket_2, self.bucket_1, self.moves)
    }

    fn check(&self, goal: u8) -> Option<BucketStats> {
        match goal {
            _ if goal == self.bucket_1 => Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::One,
                other_bucket: self.bucket_2,
            }),
            _ if goal == self.bucket_2 => Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::Two,
                other_bucket: self.bucket_1,
            }),
            _ => None,
        }
    }
}

impl PartialEq for Buckets {
    fn eq(&self, other: &Buckets) -> bool {
        self.bucket_1 == other.bucket_1 && self.bucket_2 == other.bucket_2
    }
}
impl Eq for Buckets {}
impl Hash for Buckets {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bucket_1.hash(state);
        self.bucket_2.hash(state);
    }
}
