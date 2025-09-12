use std::marker::PhantomData;

use a1::{A1, M1};
use a2::{A2, M2};
use common::Marked;

pub trait A: Marked {
    fn a_ref(&self) -> u64;
    fn a_mut(&mut self) -> bool;
    fn a_move(self) -> Self
    where
        Self: Sized;
}

trait Wrapper<T: ?Sized> {}

#[repr(transparent)]
struct Wrapped<T: ?Sized, V: ?Sized>(PhantomData<V>, T);

impl<T: ?Sized, V: ?Sized> Wrapper<Wrapped<T, V>> for T {}

trait Proxy {
    type T: ?Sized;

    fn ap_ref(t: &Self::T) -> u64;
    fn ap_mut(t: &mut Self::T) -> bool;
    fn ap_move(t: Self::T) -> Self::T
    where
        Self::T: Sized;
}

trait Wrappable: Wrapper<Self::Wrapped> + Marked {
    type Wrapped: ?Sized;
}

impl<T: ?Sized + Marked> Wrappable for T {
    type Wrapped = Wrapped<Self, <Self as Marked>::Marker>;
}

impl<T: ?Sized + Wrappable> A for T
where
    T::Wrapped: Proxy<T = T>,
{
    fn a_ref(&self) -> u64 {
        <T::Wrapped as Proxy>::ap_ref(self)
    }

    fn a_mut(&mut self) -> bool {
        <T::Wrapped as Proxy>::ap_mut(self)
    }

    fn a_move(self) -> Self
    where
        Self: Sized,
    {
        <T::Wrapped as Proxy>::ap_move(self)
    }
}

impl<T: ?Sized + A1> Proxy for Wrapped<T, M1> {
    type T = T;

    fn ap_ref(t: &Self::T) -> u64 {
        t.a1_ref()
    }

    fn ap_mut(t: &mut Self::T) -> bool {
        t.a1_mut()
    }

    fn ap_move(t: Self::T) -> Self::T
    where
        Self::T: Sized,
    {
        t.a1_move()
    }
}

impl<T: ?Sized + A2> Proxy for Wrapped<T, M2> {
    type T = T;

    fn ap_ref(t: &Self::T) -> u64 {
        t.a2_ref()
    }

    fn ap_mut(t: &mut Self::T) -> bool {
        t.a2_mut()
    }

    fn ap_move(t: Self::T) -> Self::T
    where
        Self::T: Sized,
    {
        t.a2_move()
    }
}
