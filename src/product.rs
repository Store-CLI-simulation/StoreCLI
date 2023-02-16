use crate::ProductTrait;

struct Product {
    title: String,
    cost: f32
}

impl ProductTrait for Product {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_cost(&self) -> f32 {
        self.cost
    }
}