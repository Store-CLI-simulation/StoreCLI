pub(crate) mod product;
pub(crate) mod client;
pub(crate) mod order;
pub(crate) mod basket;
pub(crate) mod product_db;
use std::io;
use client::Client;

trait ClientTrait {
    type OrderTraitType;
    fn login(&mut self, login: String, password: String);
    fn exit(&mut self);

    fn place_an_order(&self) -> Self::OrderTraitType;
    fn get_order_hystory(&self) -> Vec<Self::OrderTraitType>;

    fn deposit_balance(&mut self, count: f32);
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
    fn get_product(&self, id: usize) -> Option<<Self as AdminTrait>::ProductTraitType>;
}

trait OrderTrait {
    type ProductTraitType: ProductTrait;
    type BasketTraitType: BasketTrait<ProductTraitType = <Self as OrderTrait>::ProductTraitType>;
    fn get_products(&self) -> <Self as OrderTrait>::BasketTraitType;
}

trait ProductTrait {
    fn get_title(&self) -> String;
    fn get_cost(&self) -> f32;
}

trait BasketTrait {
    type ProductTraitType: ProductTrait;
    fn add_product(&mut self, product: <Self as BasketTrait>::ProductTraitType) -> usize;
    fn delete_product(&mut self, id: usize);
}

fn main() {
    let mut user: Client = Client::new("database.db".to_string());
    let mut buffer = String::new();
    loop {
        println!(">>>");
        io::stdin().read_line(&mut buffer).unwrap();
        println!("{0}", buffer.trim());
        // println!("endl");
        if buffer.trim() == "login".to_string() {
            let mut login: String = String::new();
            let mut password: String = String::new();
            println!("login:");
            io::stdin().read_line(&mut login).unwrap();
            println!("password:");
            io::stdin().read_line(&mut password).unwrap();
            user = Client::new_loginned(login, password, "database.db".to_string());
            println!("loginned!!!");
        }
        else if buffer.trim() == "op".to_string() {
            println!("opping");
            if user.is_loginned {
                user.is_admin = true;
                println!("You are opped");
            } else {
                println!("Login first, please");
            }
        }
        else if buffer.trim() == "unlogin".to_string() {
            if user.is_loginned {user = Client::new("database.db".to_string());}
            else {println!("Login first, please")};
        }
        else if buffer.trim() == "exit".to_string() {
            break;
        }
        buffer = "".to_string();
    }
}
