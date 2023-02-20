pub(crate) mod product;
pub(crate) mod client;
pub(crate) mod order;
pub(crate) mod basket;
pub(crate) mod product_db;
use std::io;
use basket::Basket;
use client::Client;
use std::str::FromStr;

use crate::product::Product;
trait ClientTrait {
    type OrderTraitType;
    fn login(&mut self, login: String, password: String);
    fn exit(&mut self);

    fn place_an_order(&self) -> Self::OrderTraitType;
    fn get_order_hystory(&self) -> Vec<Self::OrderTraitType>;

    fn deposit_balance(&mut self, count: f32);
    fn get_balance(&self) -> f32;
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
    let mut basket: Basket = Basket::new();
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
            if !user.is_loginned {
                println!("Login first,please");
                buffer = "".to_string();
                continue;
            }
            user.deposit_balance(f32::from_str(whitespace.next().unwrap()).unwrap());
            println!("Balance now: {}", user.get_balance());
        }
        else if cmd == "get_balance".to_string() {
            if !user.is_loginned {
                println!("Login first,please");
                buffer = "".to_string();
                continue;
            }
            println!("Balance: {}", user.get_balance());
        }
        else if cmd == "add_product".to_string() {
            if !user.is_loginned{
                println!("Please, login first!");
                buffer = "".to_string();
                continue;
            }
            let product_title: String = whitespace.next().unwrap().to_string();
            let product_uid: usize = user.get_product_db().get_uid_by_title(product_title);
            
            basket.add_product(user.get_product_db().get_product(product_uid));
        }
        else if cmd == "delete_product".to_string() {
            if !user.is_loginned{
                println!("Please, login first!");
                buffer = "".to_string();
                continue;
            }
            let title: String = whitespace.next().unwrap().to_string();
            for product_id in 0..basket.get_product_count() {
                if basket.get_product(product_id).get_title() == title {
                    basket.delete_product(product_id);
                    break;
                }
            }
        }
        else if cmd == "get_products".to_string() {
            if !user.is_loginned{
                println!("Please, login first!");
                buffer = "".to_string();
                continue;
            }
            for product_id in 0..basket.get_product_count() {
                println!("{0} \tcost: {1}", basket.get_product(product_id).get_title(), basket.get_product(product_id).get_cost());
            }
        }
        else if cmd == "order_products".to_string() {
            
        }
        else if cmd == "get_ordering_history".to_string() {
            
        }
        if user.is_admin {
            if cmd == "db_add_product".to_string() {
                let title: String = whitespace.next().unwrap().to_string();
                let cost: f32 = f32::from_str(whitespace.next().unwrap()).unwrap();

                let new_product = Product { title, cost };

                let uid = user.add_product(&new_product);
                println!("Product added!\nUID: {uid}");
            }
            else if cmd == "db_delete_product".to_string() {
                let title: String = whitespace.next().unwrap().to_string();
                let uid: usize = user.get_product_db().get_uid_by_title(title);
                user.remove_product(uid);
            }
            else if cmd == "db_edit_product".to_string() {
                let title: String = match whitespace.next() {
                    Some(value) => value,
                    None => {
                        println!("No title of editing product");
                        buffer = "".to_string();
                        continue;
                    },
                }.to_string();
                let uid: usize = user.get_product_db().get_uid_by_title(title);

                let mut selected_product: Product = user.get_product(uid).unwrap();

                let mut subbuffer: String = String::new();
                loop {
                    println!("{0}[H{0}[J", 27 as char);
                    println!("Product title: {}", selected_product.get_title());
                    println!("Product cost: {}", selected_product.get_cost());
                    println!("What change? ([T]itle/[C]ost)");
                    println!("[E for exit]\n[A for Apply]");
                    io::stdin().read_line(&mut subbuffer).unwrap();
                    if subbuffer.trim() == "T".to_string() {
                        loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Title:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    break;
                                },
                                Err(_) => continue,
                            }
                        }
                        selected_product.title = subbuffer.clone().trim().to_string();
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim() == "C".to_string() {
                        loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Cost:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    break;
                                },
                                Err(_) => continue,
                            }
                        }
                        selected_product.cost = f32::from_str(&subbuffer.trim().to_string()).unwrap();
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim() == "E".to_string() {
                        println!("Exiting");
                        break;
                    }
                    else if subbuffer.trim() == "A".to_string() {
                        println!("Applying changes");
                        user.update_product(&selected_product, uid);
                        break;
                    }
                }
            }
        }
        buffer = "".to_string();
    }
}
