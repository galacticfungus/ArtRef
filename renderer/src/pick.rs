pub struct PickManager<'a, T> {
    items: &'a mut Vec<T>,
}

impl<'a, T> PickManager<'a, T> {
    pub fn new(items: &'a mut Vec<T>) -> PickManager<T> {
        PickManager {
            items,
        }
    }

    pub fn pick_mode(&mut self, mode: T) -> &mut Self {
        self.items.push(mode);
        self
    }
}