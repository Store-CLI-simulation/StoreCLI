use crate::OrderTrait;
use crate::basket::Basket;
use crate::product::Product;

#[derive(Clone)]
pub struct Order {
    pub products: Basket
}


impl OrderTrait for Order {
    type ProductTraitType = Product;
    type BasketTraitType = Basket;
    fn get_products(&self) -> Basket {
        self.products.clone()
    }

}