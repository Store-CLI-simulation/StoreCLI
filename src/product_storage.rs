use crate::{ProductStorageTrait, product::Product};

struct ProductStorage {
    product: Product,
    count: usize
}

impl ProductStorageTrait for ProductStorage {
    type ProductTraitType = Product;
    fn get_product(&self) -> <Self as ProductStorageTrait>::ProductTraitType {
        self.product.clone()
    }

    fn get_count(&self) -> usize {
        self.count
    }
}