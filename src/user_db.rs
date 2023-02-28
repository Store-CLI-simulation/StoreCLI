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
    pub fn get_uid_by_title(&self, title: String) -> usize {
        return self.connection.query_row("SELECT uid FROM `Users` WHERE name=?", [title, ], |row| {return row.get(0)}).unwrap();
    }
}

impl Clone for UserDB {
    fn clone(&self) -> Self {
        return UserDB::new(self.filepath.clone());
    }
}

impl UserDBTrait for UserDB {
    fn add_user(&mut self, username: String, password: String) {
        self.connection.execute("INSERT INTO `Users` (name, password, balance, banned, is_admin) VALUES (?1, ?2, ?3, ?4, ?5);", (username, password, 0.0f32, 0, 0)).unwrap();
    }

    fn ban_user(&mut self, username: String) {
        self.connection.execute("INSERT OR REPLACE INTO `Users` (name, banned) VALUES (?1, ?2);", (username, 1)).unwrap();
    }
    fn is_user_exists(&self, username: String) -> bool {
        match self.connection.query_row("SELECT uid FROM `Users` WHERE name=?;", [username,], |row| {
            return row.get::<usize, u32>(0)
        }) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    fn get_password(&self, username: String) -> String {
        return
            self.connection
                .query_row("SELECT password FROM `Users` WHERE name=?;", [username, ], |row| {
                    return row.get::<usize, String>(0usize);
                }).unwrap();
    }

    fn get_balance(&self, username: String) -> f32 {
        return
            self
                .connection
                .query_row("SELECT balance FROM `Users` WHERE uid=?;", [self.get_uid_by_title(username.clone()),], |row| row.get::<usize, f32>(0))
                .unwrap();
    }
    fn set_balance(&mut self, username: String, balance: f32) {
        self
            .connection
            .execute("UPDATE `Users` SET balance=?2 WHERE uid=?1;", (self.get_uid_by_title(username), balance))
            .unwrap();
    }

    fn set_admin(&mut self, username: String, admin: usize) {
        self
            .connection
            .execute("UPDATE `Users` SET is_admin=?2 WHERE uid=?1;", (self.get_uid_by_title(username), admin))
            .unwrap();
    }
    fn get_admin(&self, username: String) -> usize {
        return 
            self
                .connection
                .query_row("SELECT is_admin FROM `Users` WHERE uid=?;", [self.get_uid_by_title(username),], |row| {
                    return row.get(0);
                })
                .unwrap();
    }

}