use crate::ProductTrait;
#[derive(Clone, PartialEq)]
pub struct Product {
    pub title: String,
    pub cost: f32
}

impl ProductTrait for Product {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_cost(&self) -> f32 {
        self.cost
    }
}