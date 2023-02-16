use crate::BasketTrait;
use crate::product::Product as CLIProduct;

#[derive(Clone)]
pub struct Basket {
    products: Vec<CLIProduct>
}

impl BasketTrait for Basket {
    type ProductTraitType = CLIProduct;

    fn add_product(&mut self, product: <Self as BasketTrait>::ProductTraitType) -> usize {
        self.products.push(product.clone());
        self.products.len()
    }

    fn delete_product(&mut self, id: usize) {
        self.products.remove(id);
    }

}
