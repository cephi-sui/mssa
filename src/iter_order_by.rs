// Provides an implementation of the nightly iter_order_by feature.
// All of this code is taken directly from the standard library,
// with the #[unstable(...)] removed.
//
// Yes, I know this is horrible.
//
// cephi: What the hell have you created...

use std::{cmp::Ordering, ops::ControlFlow};

#[inline]
fn iter_compare<A, B, F, T>(mut a: A, mut b: B, f: F) -> ControlFlow<T, Ordering>
where
    A: Iterator,
    B: Iterator,
    F: FnMut(A::Item, B::Item) -> ControlFlow<T>,
{
    #[inline]
    fn compare<'a, B, X, T>(
        b: &'a mut B,
        mut f: impl FnMut(X, B::Item) -> ControlFlow<T> + 'a,
    ) -> impl FnMut(X) -> ControlFlow<ControlFlow<T, Ordering>> + 'a
    where
        B: Iterator,
    {
        move |x| match b.next() {
            None => ControlFlow::Break(ControlFlow::Continue(Ordering::Greater)),
            Some(y) => f(x, y).map_break(ControlFlow::Break),
        }
    }

    match a.try_for_each(compare(&mut b, f)) {
        ControlFlow::Continue(()) => ControlFlow::Continue(match b.next() {
            None => Ordering::Equal,
            Some(_) => Ordering::Less,
        }),
        ControlFlow::Break(x) => x,
    }
}

pub trait MyIterOrderBy {
    fn my_cmp_by<I, F>(self, other: I, cmp: F) -> Ordering
    where
        Self: Sized + IntoIterator,
        I: IntoIterator,
        F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Ordering,
    {
        #[inline]
        fn compare<X, Y, F>(mut cmp: F) -> impl FnMut(X, Y) -> ControlFlow<Ordering>
        where
            F: FnMut(X, Y) -> Ordering,
        {
            move |x, y| match cmp(x, y) {
                Ordering::Equal => ControlFlow::Continue(()),
                non_eq => ControlFlow::Break(non_eq),
            }
        }

        match iter_compare(self.into_iter(), other.into_iter(), compare(cmp)) {
            ControlFlow::Continue(ord) => ord,
            ControlFlow::Break(ord) => ord,
        }
    }
}

impl<I: IntoIterator> MyIterOrderBy for I {}
