pub(crate) mod product;
pub(crate) mod client;
pub(crate) mod order;
pub(crate) mod basket;
pub(crate) mod product_db;
pub(crate) mod product_storage;
use std::io;
use basket::Basket;
use client::Client;
use std::str::FromStr;

use crate::{product::Product, product_storage::ProductStorage};
trait ClientTrait {
    type OrderTraitType;
    type BasketTraitType: BasketTrait;
    fn login(&mut self, login: String, password: String);
    fn exit(&mut self);

    fn place_an_order(&mut self, basket: <Self as ClientTrait>::BasketTraitType);
    fn get_order_history(&self) -> Vec<Self::OrderTraitType>;

    fn deposit_balance(&mut self, count: f32);
    fn get_balance(&self) -> f32;
}

trait ProductDBTrait {
    type ProductTraitType: ProductTrait;
    type ProductStorageTraitType: ProductStorageTrait;
    fn add_product(&mut self, product: &<Self as ProductDBTrait>::ProductStorageTraitType) -> usize;
    fn remove_product(&mut self, id: usize);
    fn update_product(&mut self, new_product: &<Self as ProductDBTrait>::ProductStorageTraitType, id: usize);
    fn get_product(&self, id: usize) -> <Self as ProductDBTrait>::ProductStorageTraitType;
}

trait AdminTrait where Self: ClientTrait{
    type ProductTraitType: ProductTrait;
    type ProductStorageTraitType: ProductStorageTrait;

    fn add_product(&mut self, product: &<Self as AdminTrait>::ProductStorageTraitType) -> usize;
    fn remove_product(&mut self, id: usize);
    fn update_product(&mut self, new_product: &<Self as AdminTrait>::ProductStorageTraitType, id: usize);
    fn get_product(&self, id: usize) -> Option<<Self as AdminTrait>::ProductStorageTraitType>;
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
trait ProductStorageTrait {
    type ProductTraitType: ProductTrait;

    fn get_product(&self) -> <Self as ProductStorageTrait>::ProductTraitType;
    fn get_count(&self) -> f32;
}
trait BasketTrait {
    type ProductTraitType: ProductTrait;
    type ProductStorageTraitType: ProductStorageTrait;
    fn add_product(&mut self, product: <Self as BasketTrait>::ProductStorageTraitType) -> usize;
    fn delete_product(&mut self, id: usize);
}

fn main() {
    println!("{0}[H{0}[J", 27 as char);
    let mut user: Client = Client::new("database.db".to_string());
    let mut username: String = "".to_string();
    let mut basket: Basket = Basket::new();
    let mut buffer = String::new();
    let mut last_output: String = "".to_string();
    loop {
        print!("[{} {}$]", username, user.get_balance());
        println!("{}", if user.is_admin {"[ADMIN]"} else {""});
        println!("{}", last_output);
        io::stdin().read_line(&mut buffer).unwrap();

        let mut whitespace = buffer.split_whitespace();
        let cmd = whitespace.next().unwrap().to_string();
        if cmd == "login".to_string() {
            username = String::new();
            let mut password: String = String::new();
            println!("login:");
            io::stdin().read_line(&mut username).unwrap();
            username = username.trim().to_string();
            println!("password:");
            io::stdin().read_line(&mut password).unwrap();
            user = Client::new_loginned(username.clone(), password.trim().to_string(), "database.db".to_string());
            println!("loginned!!!");
        }
        else if cmd == "op".to_string() {
            println!("opping");
            if user.is_loginned {
                user.is_admin = !user.is_admin;
                last_output = "You are opped".to_string();
            } else {
                last_output = "Login first, please".to_string();
            }
        }
        else if cmd == "unlogin".to_string() {
            if user.is_loginned {user = Client::new("database.db".to_string()); username = "".to_string();}
            else {last_output = "Login first, please".to_string()};
        }
        else if cmd == "exit".to_string() {
            break;
        }
        else if cmd == "help".to_string() {
            last_output = "login - открыть подпрограмму программу логина
unlogin - разлогиниться
op - перейти в админ-режим или выйти из него
exit - выйти из StoreCLI
deposit value - пополнить баланс на value
credits - просмотреть авторов
add_product title - Добавить продукт title в корзину
delete_product title - Убрать продукт title из корзины
order_products - заказать товары из корзины
get_ordering_history - получить историю заказов
админ:
db_add_product title cost - добавить продукт в базу данных
db_delete_product title - стереть продукт из базы данных
db_edit_product title - запуск подпрограммы редактирования продукта по title".to_string();
        }
        else if cmd == "credits".to_string() {
            last_output = "Авторы:
Ховрин Дмитрий Николаевич
Игорь Ротарь".to_string();
        }
        else if cmd == "deposit".to_string() {
            if !user.is_loginned {
                last_output = "Login first,please".to_string();
                buffer = "".to_string();
                continue;
            }
            user.deposit_balance(f32::from_str(whitespace.next().unwrap()).unwrap());
            last_output = format!("Balance now: {}$", user.get_balance()).to_string();
        }
        else if cmd == "get_balance".to_string() {
            if !user.is_loginned {
                last_output = "Login first, please".to_string();
                buffer = "".to_string();
                continue;
            }
            last_output = format!("Balance now: {}$", user.get_balance()).to_string();
        }
        else if cmd == "add_product".to_string() {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue;
            }
            let product_title: String = whitespace.next().unwrap().to_string();
            let product_uid: usize = user.get_product_db().get_uid_by_title(product_title);
            
            basket.add_product(ProductStorage {product: user.get_product_db().get_product(product_uid).get_product(), count: 1.0});
            last_output = "Product added to basket".to_string();
        }
        else if cmd == "delete_product".to_string() {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue;
            }
            let title: String = whitespace.next().unwrap().to_string();
            for product_id in 0..basket.get_storage_count() {
                if basket.get_storage(product_id).get_product().get_title() == title {
                    basket.delete_product(product_id);
                    last_output = "product added to basket".to_string();
                    break;
                }
            }
        }
        else if cmd == "get_products".to_string() {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue;
            }
            last_output = "".to_string();
            for product_id in 0..basket.get_storage_count() {
                last_output = format!("{0}{1} \tcost: {2}$\n", last_output,
                    basket.get_storage(product_id).get_product().get_title(),
                    basket.get_storage(product_id).get_product().get_cost()).to_string();
            }
        }
        else if cmd == "order_products".to_string() {
            let mut total_cost: f32 = 0.0;
            for id in 0..basket.get_storage_count() {
                total_cost += basket.get_storage(id).get_count() * basket.get_storage(id).get_product().get_cost();
            }
            if user.get_balance() > total_cost {
                user.place_an_order(basket.clone());
                user.pay(total_cost);
                let mut exiting: bool = false;
                for id in 0..basket.get_storage_count() {
                    let storage_0 = basket.get_storage(id);
                    let uid = user.get_product_db().get_uid_by_title(
                        storage_0.get_product().get_title()
                    );
                    let mut storage: ProductStorage = match user.get_product(uid) {
                        Some(value) => value,
                        None => {
                            println!("Error");
                            exiting = true;
                            break;
                        },
                    };
                    if !exiting {
                        storage.count -= basket.get_storage(id).get_count();
                        user.update_product(&storage.clone(), uid);
                    }
                }
                last_output = "Order placed".to_string();
            }
            else {
                last_output = "Not enough money for purpose".to_string();
            }
        }
        else if cmd == "get_ordering_history".to_string() {
            let history: Vec<order::Order> = user.get_order_history();
            last_output = "".to_string();
            for order in history {
                let basket: Basket = order.products;

                let mut total_cost: f32 = 0.0;
                for id in 0..basket.get_storage_count() {
                    total_cost += basket.get_storage(id).get_count() * basket.get_storage(id).get_product().get_cost();
                }
                last_output = format!("{0}cost: {1}$\n", last_output, total_cost);
                // panic!("{last_output}");
            }
        }
        if user.is_admin {
            if cmd == "db_add_product".to_string() {
                let title: String = whitespace.next().unwrap().to_string();
                let cost: f32 = f32::from_str(whitespace.next().unwrap()).unwrap();

                let new_product = ProductStorage {product: Product { title, cost }, count: 1.0};

                let uid = user.add_product(&new_product);
                last_output = format!("Product added!\nUID: {uid}").to_string();
            }
            else if cmd == "db_delete_product".to_string() {
                let title: String = whitespace.next().unwrap().to_string();
                let uid: usize = user.get_product_db().get_uid_by_title(title);
                user.remove_product(uid);
                last_output = "Product removed!".to_string();
            }
            else if cmd == "db_edit_product".to_string() {
                let title: String = match whitespace.next() {
                    Some(value) => value,
                    None => {
                        last_output = "No title of editing product".to_string();
                        buffer = "".to_string();
                        continue;
                    },
                }.to_string();
                let uid: usize = user.get_product_db().get_uid_by_title(title);

                let mut selected_product_storage: ProductStorage = user.get_product(uid).unwrap();

                let mut subbuffer: String = String::new();
                loop {
                    println!("{0}[H{0}[J", 27 as char);
                    println!("Product [T]itle: {}", selected_product_storage.get_product().get_title());
                    println!("Product [C]ost: {}", selected_product_storage.get_product().get_cost());
                    println!("Product [Count]: {}", selected_product_storage.get_count());
                    println!("[E]xit\n[A]pply");
                    println!("What change?");
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
                        let mut product: Product = selected_product_storage.get_product();
                        product.title = subbuffer.clone().trim().to_string();
                        selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
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
                        let mut product: Product = selected_product_storage.get_product();
                        product.cost = f32::from_str(&subbuffer.trim().to_string()).unwrap();
                        selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim().to_lowercase().to_string() == "count".to_string() {
                        loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Count:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    break;
                                },
                                Err(_) => continue,
                            }
                        }
                        selected_product_storage.count = f32::from_str(&subbuffer.trim().to_string()).unwrap();
                        // selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim() == "E".to_string() {
                        println!("Exiting");
                        break;
                    }
                    else if subbuffer.trim() == "A".to_string() {
                        println!("Applying changes");
                        user.update_product(&selected_product_storage, uid);
                        break;
                    }
                }
            }
        }
        buffer = "".to_string();
        println!("{0}[H{0}[J", 27 as char);
    }
}
