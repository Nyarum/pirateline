use crate::{database::models::*, database::schema::accounts::dsl::*, server::income_packet};
use diesel::prelude::*;

impl income_packet::Auth {
    pub fn handle(self: Self, db: &mut SqliteConnection) {
        accounts
            .limit(4)
            .select(Account::as_select())
            .load(db)
            .expect("Failed to load accounts");

        println!("Auth: {:?}", self);
    }
}
