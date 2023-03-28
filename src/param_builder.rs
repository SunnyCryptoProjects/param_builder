use postgres_types::ToSql;

use crate::{as_param::AsParam, param::Param};

#[derive(Debug, Default)]
pub struct ParamBuilder {
    v: Vec<Param>,
}

impl ParamBuilder {
    pub fn push(&mut self, t: &impl AsParam) {
        self.v.push(t.as_param());
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
        $(param_builder.extend($vs.as_params());)*
        param_builder
    }};
}
