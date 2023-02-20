use crate::BasketTrait;
use crate::product::Product as CLIProduct;
use crate::product_storage::ProductStorage;

#[derive(Clone)]
pub struct Basket {
    products: Vec<ProductStorage>
}
impl Basket {
    pub fn new () -> Basket {
        Basket { products:  vec![] }
    }
    pub fn get_product_count(&self) -> usize {
        self.products.len()
    }
    pub fn get_product(&self, id:usize) -> ProductStorage {
        self.products[id].clone()
    } 
}
impl BasketTrait for Basket {
    type ProductTraitType = CLIProduct;
    type ProductStorageTraitType = ProductStorage;

    fn add_product(&mut self, storage: <Self as BasketTrait>::ProductStorageTraitType) -> usize {
        self.products.push(storage.clone());
        self.products.len()
    }

    fn delete_product(&mut self, id: usize) {
        self.products.remove(id);
    }

}
