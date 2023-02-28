pub(crate) mod product;
pub(crate) mod client;
pub(crate) mod order;
pub(crate) mod basket;
pub(crate) mod product_db;
pub(crate) mod user_db;
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

trait UserDBTrait {
    fn add_user(&mut self, username: String, password: String);
    fn ban_user(&mut self, username: String);
    fn is_user_exists(&self, username: String) -> bool;
    fn get_password(&self, username: String) -> String;
    fn get_balance(&self, username: String) -> f32;
    fn set_balance(&mut self, username: String, balance: f32);

    fn set_admin(&mut self, username: String, admin: usize);
    fn get_admin(&self, username: String) -> usize;
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
    println!("{0}[H{0}[J", 0x1B as char);
    let mut user: Client = Client::new("database.db".to_string());
    let mut username: String = "".to_string();
    let mut basket: Basket = Basket::new();
    let mut buffer: String = String::new();
    let mut last_output: String = "".to_string();
    'main: loop {
        print!("[{} {}$]", username, user.get_balance());
        println!("{}", if user.is_admin {" \x1b[101m[ADMIN]\x1b[49m ".to_string()} else {"".to_string()});
        println!("{}", last_output);
        io::stdin().read_line(&mut buffer).unwrap();
        let b_copy = buffer.clone();
        let mut whitespace = b_copy
                                                    .split_whitespace();
        let cmd = whitespace.next().unwrap().to_string();
        if cmd.trim() == "login" {
            username = "".to_string();
            let mut password: String = "".to_string();
            println!("login:");
            io::stdin()
                    .read_line(&mut username)
                    .unwrap();
            username = username.trim().to_string();
            if !user.user_db.is_user_exists(username.clone()) {
                println!("{}", user.user_db.is_user_exists(username.clone()));
                println!("User not exists");
                username = "".to_string();
                buffer = "".to_string();
                continue 'main;
            }
            println!("password:");
            'try_get_password: for _ in 0..3 {
                io::stdin()
                        .read_line(&mut password)
                        .unwrap();
                if user.user_db.get_password(username.clone()) == password.trim().to_string() {
                    user = Client::new_loginned(username.clone(), password.trim().to_string(), "database.db".to_string());
                    println!("loginned!!!");
                    let balance = user.user_db.get_balance(username.clone()).clone();
                    if balance > 0.0f32{
                        user
                            .deposit_balance(
                                balance
                            );
                    }
                    user.is_admin = if user.user_db.get_admin(username.clone()) == 0 {false} else {true};

                    break 'try_get_password;
                } else {
                    println!("Password is wrong");
                    // buffer = "".to_string();
                    continue 'try_get_password;
                }
            }
            if !user.is_loginned {
                username = "".to_string();
            }
        }
        else if cmd.trim() == "register" {
            
            username = "".to_string();
            let mut password: String = "".to_string();
            'try_get_login: loop {
                println!("login:");
                match io::stdin().read_line(&mut username) {
                    Ok(_) => {
                        if user.user_db.is_user_exists(username.clone()) {
                            println!("Login is used");
                            continue 'try_get_login
                        }
                        else {
                            break 'try_get_login
                        };
                    },
                    Err(_) => continue 'try_get_login,
                };
            }
            username = username.trim().to_string();
            'try_get_password: loop {
                println!("password:");
                match io::stdin().read_line(&mut password) {
                    Ok(_) => break 'try_get_password,
                    Err(_) => continue 'try_get_password,
                };
            }
            password = password.trim().to_string();
            println!("uname: {} pass: {}", username.clone(), password.clone());
            user
                .user_db
                .add_user(username.clone(), password.clone());
            user
                .login(username.clone(), password.clone());
            io::stdin().read_line(&mut buffer).unwrap();
        }
        else if cmd.trim() == "op" {
            println!("opping");
            if user.is_loginned {
                user.is_admin = !user.is_admin;
                last_output = format!("You are {}opped", if user.is_admin {""} else {"de"}).to_string();
                user
                    .user_db
                    .set_admin(username.clone(), if user.is_admin {1} else {0});
            } else {
                last_output = "Login first, please".to_string();
            }
        }
        else if cmd.trim() == "unlogin" {
            if user.is_loginned {user = Client::new("database.db".to_string()); username = "".to_string();}
            else {last_output = "Login first, please".to_string()};
        }
        else if cmd.trim() == "exit" {
            break 'main;
        }
        else if cmd.trim() == "help" {
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
        else if cmd.trim() == "credits" {
            last_output = "Авторы:
Ховрин Дмитрий Николаевич
Игорь Ротарь".to_string();
        }
        else if cmd.trim() == "deposit" {
            if !user.is_loginned {
                last_output = "Login first,please".to_string();
                buffer = "".to_string();
                continue 'main;
            }
            user.deposit_balance(f32::from_str(whitespace.next().unwrap()).unwrap());
            last_output = format!("Balance now: {}$", user.get_balance()).to_string();
        }
        else if cmd.trim() == "get_balance" {
            if !user.is_loginned {
                last_output = "Login first, please".to_string();
                buffer = "".to_string();
                continue 'main;
            }
            last_output = format!("Balance now: {}$", user.get_balance()).to_string();
        }
        else if cmd.trim() == "add_product" {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue 'main;
            }
            let product_title: String = whitespace.next().unwrap().to_string();
            let product_uid: usize = user.get_product_db().get_uid_by_title(product_title);
            
            basket.add_product(ProductStorage {product: user.get_product_db().get_product(product_uid).get_product(), count: 1.0});
            last_output = "Product added to basket".to_string();
        }
        else if cmd.trim() == "delete_product" {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue;
            }
            let title: String = whitespace.next().unwrap().to_string();
            'product_list: for product_id in 0..basket.get_storage_count() {
                if basket.get_storage(product_id).get_product().get_title() == title {
                    basket.delete_product(product_id);
                    last_output = "product added to basket".to_string();
                    break 'product_list;
                }
            }
        }
        else if cmd.trim() == "get_products" {
            if !user.is_loginned{
                last_output = "Please, login first!".to_string();
                buffer = "".to_string();
                continue 'main;
            }
            last_output = "".to_string();
            for product_id in 0..basket.get_storage_count() {
                last_output = format!("{0}{1} \tcost: {2}$\n", last_output,
                    basket.get_storage(product_id).get_product().get_title(),
                    basket.get_storage(product_id).get_product().get_cost()).to_string();
            }
        }
        else if cmd.trim() == "order_products" {
            let mut total_cost: f32 = 0.0;
            for id in 0..basket.get_storage_count() {
                total_cost += basket.get_storage(id).get_count() * basket.get_storage(id).get_product().get_cost();
            }
            if user.get_balance() > total_cost {
                user.place_an_order(basket.clone());
                user.pay(total_cost);
                'cost_counting: for id in 0..basket.get_storage_count() {
                    let storage_0 = basket.get_storage(id);
                    let uid = user.get_product_db().get_uid_by_title(
                        storage_0.get_product().get_title()
                    );
                    let mut storage: ProductStorage = match user.get_product(uid) {
                        Some(value) => value,
                        None => {
                            println!("Error");
                            break 'cost_counting;
                        },
                    };
                    storage.count -= basket.get_storage(id).get_count();
                    user.update_product(&storage.clone(), uid);
                }
                last_output = "Order placed".to_string();
                basket = Basket::new();
            }
            else {
                last_output = "Not enough money for purpose".to_string();
            }
        }
        else if cmd.trim() == "get_ordering_history" {
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
            if cmd.trim() == "db_add_product" {
                let title: String = whitespace.next().unwrap().to_string();
                let cost: f32 = f32::from_str(whitespace.next().unwrap()).unwrap();

                let new_product = ProductStorage {product: Product { title, cost }, count: 1.0};

                let uid = user.add_product(&new_product);
                last_output = format!("Product added!\nUID: {uid}").to_string();
            }
            else if cmd.trim() == "db_delete_product" {
                let title: String = whitespace.next().unwrap().to_string();
                let uid: usize = user.get_product_db().get_uid_by_title(title);
                user.remove_product(uid);
                last_output = "Product removed!".to_string();
            }
            else if cmd.trim() == "db_edit_product" {
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
                'db_edit_product_main_menu: loop {
                    println!("{0}[H{0}[J", 27 as char);
                    println!("Product [T]itle: {}", selected_product_storage.get_product().get_title());
                    println!("Product [C]ost: {}", selected_product_storage.get_product().get_cost());
                    println!("Product [Count]: {}", selected_product_storage.get_count());
                    println!("[E]xit\n[A]pply");
                    println!("What change?");
                    io::stdin().read_line(&mut subbuffer).unwrap();
                    if subbuffer.trim() == "T".to_string() {
                        'db_edit_product_get_new_title: loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Title:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    if subbuffer.trim() == "E"{
                                        continue 'db_edit_product_main_menu;
                                    }
                                    else {
                                        break 'db_edit_product_get_new_title;
                                    }
                                },
                                Err(_) => continue 'db_edit_product_get_new_title
                            }
                        }
                        let mut product: Product = selected_product_storage.get_product();
                        product.title = subbuffer.clone().trim().to_string();
                        selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim() == "C".to_string() {
                        'db_edit_product_get_new_cost: loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Cost:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    if subbuffer.trim() == "E" {
                                        continue 'db_edit_product_main_menu;
                                    }
                                    else {
                                        break 'db_edit_product_get_new_cost;
                                    }
                                },
                                Err(_) => continue 'db_edit_product_get_new_cost,
                            }
                        }
                        let mut product: Product = selected_product_storage.get_product();
                        product.cost = f32::from_str(&subbuffer.trim().to_string()).unwrap();
                        selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim().to_lowercase().to_string() == "count".to_string() {
                        'db_edit_product_get_new_count: loop {
                            subbuffer = "".to_string();
                            println!("{0}[H{0}[J", 27 as char);
                            println!("[E to stop]");
                            println!("New Count:");
                            match io::stdin().read_line(&mut subbuffer) {
                                Ok(_) => {
                                    if subbuffer.trim() == "E" {
                                        continue 'db_edit_product_main_menu;
                                    } 
                                    else {
                                        break 'db_edit_product_get_new_count;
                                    }
                                },
                                Err(_) => continue 'db_edit_product_get_new_count,
                            }
                        }
                        selected_product_storage.count = f32::from_str(&subbuffer.trim().to_string()).unwrap();
                        // selected_product_storage = ProductStorage {product, count: selected_product_storage.get_count()};
                        subbuffer = "".to_string();
                    }
                    else if subbuffer.trim() == "E".to_string() {
                        println!("Exiting");
                        break 'db_edit_product_main_menu;
                    }
                    else if subbuffer.trim() == "A".to_string() {
                        println!("Applying changes");
                        user.update_product(&selected_product_storage, uid);
                        break 'db_edit_product_main_menu;
                    }
                }
            }
        }
        buffer = "".to_string();
        println!("{0}[H{0}[J", 27 as char);
    }
}
