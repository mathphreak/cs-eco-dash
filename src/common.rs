use std::marker;

pub trait TakesUpdates<T> : marker::Send {
    fn update(&mut self, data: T);
}
