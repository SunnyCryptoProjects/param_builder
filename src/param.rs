use postgres_types::ToSql;

pub type Param = Box<dyn ToSql + Sync + Send>;
