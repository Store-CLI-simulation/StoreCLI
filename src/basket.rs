use crate::BasketTrait;
use crate::product::Product as CLIProduct;

#[derive(Clone)]
pub struct Basket {
    products: Vec<CLIProduct>
}
impl Basket {
    pub fn new () -> Basket {
        Basket { products:  vec![] }
    }
    pub fn get_product_count(&self) -> usize {
        self.products.len()
    }
    pub fn get_product(&self, id:usize) -> CLIProduct {
        self.products[id].clone()
    } 
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
