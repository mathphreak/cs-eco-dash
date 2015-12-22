pub trait TakesUpdates<T> : Send {
    fn update(&mut self, data: &T);
}
