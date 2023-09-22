pub trait Marker<T: ?Sized> {}

pub trait Marked {
    type Marker: ?Sized + Marker<Self>;
}
