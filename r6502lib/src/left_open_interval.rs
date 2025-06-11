#![allow(unused)]

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct LeftOpenInterval<T: Copy + Debug + Ord> {
    start: T,
    end: T,
}

impl<T: Copy + Debug + Ord> LeftOpenInterval<T> {
    pub fn new(start: T, end: T) -> Self {
        assert!(end >= start);
        Self { start, end }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }

    pub fn contains(&self, other: T) -> bool {
        other >= self.start && other <= self.end
    }

    pub fn segment(intervals: &[LeftOpenInterval<T>]) -> Vec<LeftOpenInterval<T>> {
        // Collect all boundary points
        let mut boundaries = Vec::new();
        for interval in intervals {
            boundaries.push(interval.start);
            boundaries.push(interval.end);
        }

        // Sort and remove duplicates
        boundaries.sort();
        boundaries.dedup();

        let mut result = Vec::new();

        // Create segments between consecutive boundaries
        for i in 0..boundaries.len() - 1 {
            let start = boundaries[i];
            let end = boundaries[i + 1];

            // Add the main segment
            result.push(LeftOpenInterval { start, end });

            // Check if we need a zero-length segment at the end boundary
            // This happens when the end point is also a start point of another interval
            if i < boundaries.len() - 2 {
                let next_boundary = boundaries[i + 1];

                // Add zero-length segment if this boundary point is significant
                if Self::is_boundary_significant(next_boundary, intervals) {
                    result.push(LeftOpenInterval {
                        start: next_boundary,
                        end: next_boundary,
                    });
                }
            }
        }

        result
    }

    fn is_boundary_significant(point: T, intervals: &[LeftOpenInterval<T>]) -> bool {
        // A boundary is significant if it's both an end point of one interval and start of another
        let is_end_point = intervals.iter().any(|r| r.end == point);
        let is_start_point = intervals.iter().any(|r| r.start == point);
        is_end_point && is_start_point
    }
}

#[cfg(test)]
mod tests {
    use crate::LeftOpenInterval;

    #[test]
    fn basics() {
        let intervals = vec![
            LeftOpenInterval::new(5, 10),
            LeftOpenInterval::new(20, 50),
            LeftOpenInterval::new(0, 7),
            LeftOpenInterval::new(10, 60),
        ];
        let result = LeftOpenInterval::segment(&intervals);

        assert_eq!(
            vec![
                LeftOpenInterval::new(0, 5),
                LeftOpenInterval::new(5, 7),
                LeftOpenInterval::new(7, 10),
                LeftOpenInterval::new(10, 10),
                LeftOpenInterval::new(10, 20),
                LeftOpenInterval::new(20, 50),
                LeftOpenInterval::new(50, 60)
            ],
            result
        );
    }
}
