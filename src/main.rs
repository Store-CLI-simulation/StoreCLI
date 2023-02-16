use std::io;

trait ClientTrait {
    type OrderTraitType;
    fn login(&mut self);
    fn exit(&mut self);

    fn place_an_order(&self) -> Self::OrderTraitType;
    fn get_order_hystory(&self) -> Vec<Self::OrderTraitType>;

    fn deposit_balance(&mut self, count: usize);
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
