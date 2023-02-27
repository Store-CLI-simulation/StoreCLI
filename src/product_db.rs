use rusqlite::Connection;

use crate::ProductStorageTrait;
use crate::product_storage::ProductStorage;
use crate::{ProductDBTrait, product::Product as CLIProduct, ProductTrait};


pub struct ProductDB {
    filepath: String,
    connection: Connection
}
impl Clone for ProductDB {
    fn clone(&self) -> Self {
        return ProductDB::new(self.filepath.clone());
    }
}
impl ProductDB {
    pub fn new(filepath: String) -> Self {
        let product_db: ProductDB = ProductDB { filepath: filepath.clone(), connection: Connection::open(filepath).unwrap() };
        // product_db.connection = Connection::open(filepath).unwrap();

        product_db.connection
            .execute(
            "CREATE TABLE IF NOT EXISTS `Products`  (
                uid INTEGER PRIMARY KEY,
                title TEXT UNIQUE NOT NULL,
                cost REAL NOT NULL,
                count REAL NOT NULL
            );",
            ())
            .unwrap();

        return product_db;
    }
    pub fn get_uid_by_title(&self, title: String) -> usize {
        return self.connection.query_row("SELECT uid FROM `Products` WHERE title=?", [title, ], |row| {return row.get(0)}).unwrap();
    }
}

impl ProductDBTrait for ProductDB {
    type ProductTraitType = CLIProduct;
    type ProductStorageTraitType = ProductStorage;

    fn add_product(&mut self, storage: &<Self as ProductDBTrait>::ProductStorageTraitType) -> usize {
        self.connection
            .execute("INSERT INTO `Products` (title, cost, count) VALUES (?1, ?2, ?3)",
                (
                    storage.get_product().get_title(),
                    storage.get_product().get_cost(),
                    storage.get_count()
                ))
            .unwrap();
        
        return self.connection.last_insert_rowid() as usize;
    }

    fn remove_product(&mut self, id: usize) {
        self.connection
            .execute("DELETE FROM `Products` WHERE uid=?1;", (id, )).unwrap();
    }

    fn update_product(&mut self, new_storage: &<Self as ProductDBTrait>::ProductStorageTraitType, id: usize) {
        self.connection
            .execute("INSERT OR REPLACE INTO `PRODUCTS` (uid, title, cost, count) VALUES (?1, ?2, ?3, ?4)", (
                id,
                new_storage.get_product().get_title(),
                new_storage.get_product().get_cost(),
                new_storage.get_count()
            ))
            .unwrap();
    }

    fn get_product(&self, id: usize) -> <Self as ProductDBTrait>::ProductStorageTraitType {
        return 
            self
                .connection
                .query_row("SELECT title, cost, count FROM `Products` WHERE uid=?",
                [id, ],
                |row| {
                    Ok(ProductStorage {
                        product: CLIProduct {
                            title: row.get::<usize, String>(0usize).unwrap(),
                            cost: row.get::<usize, f32>(1usize).unwrap(),
                        },
                        count: row.get::<usize, f32>(2usize).unwrap()
                    }
                )})
                .unwrap();
    }
}