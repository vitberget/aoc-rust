use std::ops::RangeInclusive;

pub(crate) trait StartEnd<T: ?Sized> {
    fn the_start(&self) -> &T;
    fn the_end(&self) -> &T;
}

impl<T> StartEnd<T> for RangeInclusive<T> {
    fn the_start(&self) -> &T { self.start() }
    fn the_end(&self) -> &T { self.end() }
}