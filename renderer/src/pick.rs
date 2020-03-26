pub struct PickManager<'a, T, U> {
    items: &'a mut Vec<T>,
    available_items: &'a [U],
    output: &'a mut U,
}

impl<'a, T, U: PartialEq> PickManager<'a, T, U> 
    where U: From<&'a T> {
    pub fn new(items: &'a mut Vec<T>, output: &'a mut U, available_items: &'a [U]) -> PickManager<'a, T, U> {
        PickManager {
            items,
            available_items,
            output,
        }
    }

    pub fn pick(&mut self, mode: T) -> &mut Self {
        self.items.push(mode);
        self
    }

    pub fn get_first_available(mut self) -> () {
        for item in self.items.iter() {
            let actual_item: U = item.into();
            if self.available_items.contains(&actual_item) {
                *self.output = actual_item;
                break;
            }
        }
    }
}