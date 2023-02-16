mod product;
use std::io;


trait ClientTrait {
    type OrderTraitType;
    fn login(&mut self);
    fn exit(&mut self);

    fn place_an_order(&self) -> Self::OrderTraitType;
    fn get_order_hystory(&self) -> Vec<Self::OrderTraitType>;

    fn deposit_balance(&mut self, count: usize);
}

trait ProductDBTrait {
    type ProductTraitType: ProductTrait;

    fn add_product(&mut self, product: &<Self as ProductDBTrait>::ProductTraitType) -> usize;
    fn remove_product(&mut self, id: usize);
    fn update_product(&mut self, new_product: &<Self as ProductDBTrait>::ProductTraitType, id: usize);
    fn get_product(&self, id: usize) -> <Self as ProductDBTrait>::ProductTraitType;
}

trait AdminTrait where Self: ClientTrait{
    type ProductTraitType: ProductTrait;

    fn add_product(&mut self, product: &<Self as AdminTrait>::ProductTraitType) -> usize;
    fn remove_product(&mut self, id: usize);
    fn update_product(&mut self, new_product: &<Self as AdminTrait>::ProductTraitType, id: usize);
    fn get_product(&self, id: usize) -> <Self as AdminTrait>::ProductTraitType;
}

trait OrderTrait {
    fn get_products(&self) -> dyn BasketTrait;
}

trait ProductTrait {
    fn get_title(&self) -> String;
    fn get_cost(&self) -> f32;
}

trait BasketTrait {
    fn add_product(&mut self, product: dyn ProductTrait) -> usize;
    fn delete_product(&mut self, id: usize);
}

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer == "exit".to_string() {
            break;
        }
    }
}
