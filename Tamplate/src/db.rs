use crate::error_handler::CustomError;
use diesel::pg::pgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::new;
use diesel::embed_migrations;

type Pool = r2d2::Pool<ConnectionMananger<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static!{
    static ref POOL:Pool = {
        let db_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create pool")
    };
}

pub fn connection() -> Result<DbConnection,CustomError>{
    POOL.get().amp_err(|e| CustomError::new(500,
                    format!("Failed to get DB connection from pool:{}",e)
    ))
}

pub fn init(){
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get DB connection from pool");;
    embedded_migrations::run(&conn).unwrap();
}