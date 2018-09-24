use std::collections::VecDeque;
use std::collections::HashSet;

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
    let mut buf = VecDeque::new();
    let mut checked = HashSet::new();

    let start = match start_bucket {
        &Bucket::One => ((capacity_1, 0), 1),
        &Bucket::Two => ((0, capacity_2), 1),
    };
    buf.push_back(start);
    match start_bucket {
        &Bucket::One => checked.insert((0, capacity_2)),
        &Bucket::Two => checked.insert((capacity_1, 0)),
    };

    while let Some((b, moves)) = buf.pop_front() {
        if checked.contains(&b) { continue; }
        checked.insert(b);

        if b.0 == goal {
            return BucketStats { moves, goal_bucket: Bucket::One, other_bucket: b.1 }
        }
        if b.1 == goal {
            return BucketStats { moves, goal_bucket: Bucket::Two, other_bucket: b.0 }
        }
        buf.push_back((move_to_one(b, capacity_1), moves + 1));
        buf.push_back((move_to_two(b, capacity_2), moves + 1));
        buf.push_back(((b.0, 0), moves + 1));
        buf.push_back(((0, b.1), moves + 1));
        buf.push_back(((b.0.max(capacity_1), b.1), moves + 1));
        buf.push_back(((b.0, b.1.max(capacity_2)), moves + 1));
    }
    return BucketStats { moves: 0, goal_bucket: Bucket::One, other_bucket: 0 }
}

fn move_to_one(bucket: (u8, u8), capacity_1: u8) -> (u8, u8) {
    let space_1 = (capacity_1 - bucket.0).min(bucket.1);
    (bucket.0 + space_1, bucket.1 - space_1)
}

fn move_to_two(bucket: (u8, u8), capacity_2: u8) -> (u8, u8) {
    let moved = move_to_one((bucket.1, bucket.0), capacity_2);
    (moved.1, moved.0)
}
