use common::{Marked, Marker};

pub struct M2;

pub trait A2: Marked<Marker = M2> {
    fn a2_ref(&self) -> u64;
    fn a2_mut(&mut self) -> bool;
    fn a2_move(self) -> Self
    where
        Self: Sized;
}

impl<T: ?Sized + A2> Marker<T> for M2 {}
