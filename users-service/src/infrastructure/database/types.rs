use sea_orm::DatabaseConnection;
use sqlx::postgres::PgPool;

pub enum DatabaseConn {
    SeaOrm(DatabaseConnection),
    Sqlx(PgPool),
}
