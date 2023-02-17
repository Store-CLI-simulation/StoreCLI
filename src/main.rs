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

        // deposit value - пополнить баланс на value
        // add_product title - Добавить продукт title в корзину
        // delete_product title - Убрать продукт title из корзины
        // order_products - заказать товары из корзины
        // get_ordering_history - получить историю заказов
        // админ:
        // db_add_product title cost - добавить продукт в базу данных
        // db_delete_product title - стереть продукт из базы данных
        // db_edit_product title - запуск подпрограммы редактирования продукта по title
        let mut whitespace = buffer.split_whitespace();
        let cmd = whitespace.next().unwrap().to_string();
        if cmd == "login".to_string() {
            let mut login: String = String::new();
            let mut password: String = String::new();
            println!("login:");
            io::stdin().read_line(&mut login).unwrap();
            println!("password:");
            io::stdin().read_line(&mut password).unwrap();
            user = Client::new_loginned(login, password, "database.db".to_string());
            println!("loginned!!!");
        }
        else if cmd == "op".to_string() {
            println!("opping");
            if user.is_loginned {
                user.is_admin = true;
                println!("You are opped");
            } else {
                println!("Login first, please");
            }
        }
        else if cmd == "unlogin".to_string() {
            if user.is_loginned {user = Client::new("database.db".to_string());}
            else {println!("Login first, please")};
        }
        else if cmd == "exit".to_string() {
            break;
        }
        else if cmd == "deposit".to_string() {
            
        }
        else if cmd == "add_product".to_string() {
            let product_title: String = whitespace.next().unwrap().to_string();
            
        }
        else if cmd == "delete_product".to_string() {
            
        }
        else if cmd == "order_products".to_string() {
            
        }
        else if cmd == "get_ordering_history".to_string() {
            
        }
        if user.is_admin {
            if cmd == "db_add_product".to_string() {
            
            }
            else if cmd == "db_delete_product".to_string() {
            
            }
            else if cmd == "db_edit_product".to_string() {
            
            }
        }
        buffer = "".to_string();
    }
}
