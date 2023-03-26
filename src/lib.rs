use postgres_types::ToSql;

pub type Param = Box<dyn ToSql + Sync + Send>;

#[derive(Debug, Default)]
pub struct ParamBuilder {
    v: Vec<Param>,
}

impl ParamBuilder {
    pub fn push<T>(&mut self, t: &T)
    where
        T: ToSql + Sync + Send + Clone + 'static,
    {
        self.v.push(Box::new(t.clone()));
    }

    pub fn extend(&mut self, rhs: ParamBuilder) {
        self.v.extend(rhs.v);
    }

    pub fn build<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        self.v
            .iter()
            .map(|v| v.as_ref() as &(dyn ToSql + Sync))
            .collect()
    }
}

#[macro_export]
macro_rules! params {
    () => {{
        ::param_builder::param_builder::ParamBuilder::default()
    }};
    ($($vs:expr),*) => {{
        let mut param_builder = ::param_builder::param_builder::ParamBuilder::default();
        $(param_builder.extend($vs);)*
        param_builder
    }};
}
