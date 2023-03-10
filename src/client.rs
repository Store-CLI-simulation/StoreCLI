use crate::basket::Basket;
use crate::product_storage::ProductStorage;
use crate::user_db::UserDB;
use crate::{AdminTrait, ClientTrait, ProductDBTrait, UserDBTrait};
use crate::order::Order;
use crate::product::Product as CLIProduct;
use crate::product_db::ProductDB;

pub struct Client {
    pub is_loginned: bool,
    pub is_admin: bool,
    balance: f32,
    login: String,
    password: String,
    order_history: Vec<Order>,
    product_db: ProductDB,
    pub user_db: UserDB
}

impl Client {
    pub fn new(filepath: String) -> Self {
        Client {
            is_loginned: false,
            is_admin: false,
            balance: 0.0f32,
            login: "".to_string(),
            password: "".to_string(),
            order_history: Default::default(),
            product_db: ProductDB::new(filepath.clone()),
            user_db: UserDB::new(filepath.clone())
        }
    }
    pub fn new_loginned(login: String, password: String, filepath: String) -> Self {
        Client {
            is_loginned: true,
            is_admin: false,
            balance: 0.0f32,
            login: login,
            password: password,
            order_history: Default::default(),
            product_db: ProductDB::new(filepath.clone()),
            user_db: UserDB::new(filepath.clone())
        }
    }
    pub fn get_product_db(&self) -> ProductDB {
        return self.product_db.clone();
    }
    pub fn pay(&mut self, cost: f32) {
        self.balance -= cost;
        self.user_db.set_balance(self.login.clone(), self.balance);
    }
}

impl ClientTrait for Client {
    type OrderTraitType = Order;
    type BasketTraitType = Basket;
    fn login(&mut self, login: String, password: String) {
        self.login = login;
        self.password = password;
    }

    fn exit(&mut self) {
        self.login = "".to_string();
        self.password = "".to_string();
        self.is_loginned = false;
    }

    fn place_an_order(&mut self, basket: Basket) {
        let order: Order = Order{products: basket};
        self.order_history.push(order);
    }

    fn get_order_history(&self) -> Vec<Self::OrderTraitType> {
        self.order_history.clone()
    }

    fn deposit_balance(&mut self, count: f32) {
        self.balance += count;
        self.user_db.set_balance(self.login.clone(), self.balance);
    }

    fn get_balance(&self) -> f32 {
        return self.balance;
    }
}

impl AdminTrait for Client {
    type ProductTraitType = CLIProduct;
    type ProductStorageTraitType = ProductStorage;

    fn add_product(&mut self, product: &<Self as AdminTrait>::ProductStorageTraitType) -> usize {
        if !self.is_admin {
            return 0;
        }
        return self.product_db.add_product(product);
    }

    fn remove_product(&mut self, id: usize) {
        if !self.is_admin {
            return;
        }
        self.product_db.remove_product(id);
    }

    fn update_product(&mut self, new_product: &<Self as AdminTrait>::ProductStorageTraitType, id: usize) {
        if !self.is_admin {
            return;
        }
        self.product_db.update_product(new_product, id);
    }

    fn get_product(&self, id: usize) -> Option<ProductStorage> {
        return Some(self.product_db.get_product(id));
    }
}
