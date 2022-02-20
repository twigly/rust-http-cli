use std::result::Result;

#[derive(Clone)]
pub struct FilterOkIterator<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P, A, E> Iterator for FilterOkIterator<I, P>
where
    P: FnMut(&A) -> bool,
    I: Iterator<Item = Result<A, E>>,
{
    type Item = Result<A, E>;

    #[inline]
    fn next(&mut self) -> Option<Result<A, E>> {
        for x in self.iter.by_ref() {
            match x {
                Ok(xx) => {
                    if (self.predicate)(&xx) {
                        return Some(Ok(xx));
                    }
                }
                Err(_) => return Some(x),
            }
        }
        None
    }
}

pub trait FilterOkTrait {
    fn filter_ok<P, A, E>(self, predicate: P) -> FilterOkIterator<Self, P>
    where
        Self: Sized + Iterator<Item = Result<A, E>>,
        P: FnMut(&A) -> bool,
    {
        FilterOkIterator { iter: self, predicate }
    }
}

impl<I, T, E> FilterOkTrait for I where I: Sized + Iterator<Item = Result<T, E>> {}
