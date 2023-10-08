use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

type PostgresPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
    connection: PostgresPool,
}

impl Db {
    pub fn new(host: &str) -> Db {
        let manager = ConnectionManager::<PgConnection>::new(host);

        Db {
            connection: Pool::new(manager).expect("Postgres connection pool could not be created"),
        }
    }

    pub fn apply<T>(&self, clojure: impl Fn(&mut PgConnection) -> T) -> T {
        clojure(&mut self.connection.get().unwrap())
    }
}
