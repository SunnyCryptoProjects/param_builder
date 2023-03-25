use tokio_postgres::types::ToSql;

pub type Param = Box<dyn ToSql + Sync + Send>;
