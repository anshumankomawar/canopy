use sqlx::postgres::PgPool;

/*
 * `State` represents the application state, holding shared resources.
 *
 * - `pool` [PgPool]: The PostgreSQL connection pool for database operations.
 */
#[derive(Debug, Clone, Builder)]
pub struct State {
    pub pool: PgPool,
}
