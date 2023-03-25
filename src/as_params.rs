use crate::{as_param::AsParam, param_builder::ParamBuilder};

pub trait AsParams {
    fn as_params(&self) -> ParamBuilder;
}

impl<T> AsParams for T
where
    T: AsParam,
{
    fn as_params(&self) -> ParamBuilder {
        let mut param_builder = ParamBuilder::default();
        param_builder.push(self);
        param_builder
    }
}
