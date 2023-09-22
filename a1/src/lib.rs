use common::{Marked, Marker};

pub struct M1;

pub trait A1: Marked<Marker = M1> {
    fn a1_ref(&self) -> u64;
    fn a1_mut(&mut self) -> bool;
    fn a1_move(self) -> Self
    where
        Self: Sized;
}

impl<T: ?Sized + A1> Marker<T> for M1 {}
