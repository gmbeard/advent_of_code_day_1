pub trait AsNeighbour : Iterator
    where Self: Sized
{
    fn as_neighbour(self) -> NeighbourIter<Self>;
}

impl<I: Iterator> AsNeighbour for I {
    fn as_neighbour(mut self) -> NeighbourIter<Self> {
        let current = self.next();
        NeighbourIter {
            iter: self,
            current: current,
            first: None,
        }
    }
}

pub struct NeighbourIter<I: Iterator> {
    iter: I,
    current: Option<I::Item>,
    first: Option<I::Item>,
}

impl<I: Iterator> Iterator for NeighbourIter<I> 
    where I::Item: Copy + Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let (c, n) = self.current.take()
            .map(|c| {
                if self.first.is_none() {
                    self.first = Some(c);
                }

                (Some(c), self.iter.next())
            })
            .unwrap_or_else(|| (None, None));

        match (c, n) {
            (Some(this), Some(next)) => {
                self.current = Some(next);
                Some((this, next))
            },
            (Some(this), None) => Some((this, self.first.unwrap())),
            _ => None
        }
    }
}

#[cfg(test)]
mod as_neighbour_should {
    use super::*;

    #[test]
    fn produce_this_and_neighbour_values() {
        assert_eq!(
            vec![(1, 2), (2, 3), (3, 1), (1, 1)],
            vec![1, 2, 3, 1].into_iter()
                .as_neighbour()
                .collect::<Vec<_>>())
    }
}
