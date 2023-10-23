//! 341. Flatten Nested List Iterator

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    flattened: Vec<i32>,
    index: usize,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut flattened = Vec::new();
        Self::flatten(&nested_list, &mut flattened);

        Self {
            flattened,
            index: 0,
        }
    }

    fn flatten(nested: &[NestedInteger], result: &mut Vec<i32>) {
        for ni in nested {
            match ni {
                NestedInteger::Int(val) => result.push(*val),
                NestedInteger::List(list) => Self::flatten(list, result),
            }
        }
    }

    fn next(&mut self) -> i32 {
        let val = self.flattened[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.flattened.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test() {
        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 2);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), false);

        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![NestedInteger::Int(4), NestedInteger::List(vec![NestedInteger::Int(6)])]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 4);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 6);
        assert_eq!(iterator.has_next(), false);

        let nested_list = vec![
            NestedInteger::Int(1),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), false);

        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(2)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.has_next(), true);
        assert_eq!(iterator.next(), 2);
        assert_eq!(iterator.has_next(), false);
    }
}