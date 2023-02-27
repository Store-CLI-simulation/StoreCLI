use rusqlite::{self, Connection};

use crate::{UserDBTrait};

pub struct UserDB {
    filepath: String,
    connection: Connection
}

impl UserDB {
    pub fn new(filepath: String) -> UserDB {
        let product_db: UserDB = UserDB { filepath: filepath.clone(), connection: Connection::open(filepath).unwrap() };
        
        product_db.connection
            .execute("CREATE TABLE IF NOT EXISTS `Users` (
                uid         INTEGER PRIMARY KEY,
                name        TEXT NOT NULL UNIQUE,
                password    TEXT NOT NULL,
                balance     REAL NOT NULL,
                banned      INTEGER NOT NULL,
                is_admin    INTEGER NOT NULL
            );", ()).unwrap();

        return product_db;
    }
}

impl Clone for UserDB {
    fn clone(&self) -> Self {
        return UserDB::new(self.filepath.clone());
    }
}

impl UserDBTrait for UserDB {
    fn add_user(&mut self, username: String, password: String) {
        self.connection.execute("INSERT INTO `Users` (name, password, balance, banned, is_admin) VALUES (?1, ?2, ?3, ?4, ?5)", (username, password, 0.0f32, 0, 0)).unwrap();
    }

    fn ban_user(&mut self, username: String) {
        self.connection.execute("INSERT OR REPLACE INTO `Users` (name, banned) VALUES (?1, ?2)", (username, 1)).unwrap();
    }
    fn is_user_exists(&self, username: String) -> bool {
        match self.connection.query_row("SELECT uid FROM `Users` WHER name=?", [username,], |row| {
            return row.get::<usize, u32>(0)
        }) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    fn get_password(&mut self, username: String) -> String {
        return
            self.connection
                .query_row("SELECT password FROM `Users` WHERE name=?", [username, ], |row| {
                    return row.get(0);
                }).unwrap();
    }

    fn get_balance(&self, username: String) -> f32 {
        return
            self
                .connection
                .query_row("SELECT balance FROM `Users` WHERE name=?", [username,], |row| row.get(0))
                .unwrap();
    }
    fn set_balance(&mut self, username: String, balance: f32) {
        self
            .connection
            .execute("INSERT OR REPLACE INTO `Users` (name, balance) VALUES (?1, ?2)", (username, balance)).unwrap();
    }

}