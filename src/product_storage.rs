use std::ops::{Sub, Add};

use crate::{ProductStorageTrait, product::Product};

#[derive(Clone, Debug)]
pub struct ProductStorage {
    pub(crate) product: Product,
    pub(crate) count: f32
}
impl Add<f32> for ProductStorage {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mut result = self.clone();
        result.count += rhs;
        return result;
    }
}

impl Sub<f32> for ProductStorage {
    type Output = ProductStorage;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut result = self.clone();
        result.count -= rhs;
        return result;
    }
}
impl ProductStorageTrait for ProductStorage {
    type ProductTraitType = Product;
    fn get_product(&self) -> <Self as ProductStorageTrait>::ProductTraitType {
        self.product.clone()
    }

    fn get_count(&self) -> f32 {
        self.count
    }
}