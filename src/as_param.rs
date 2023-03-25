use crate::param::Param;

pub trait AsParam {
    fn as_param(&self) -> Param;
}

impl AsParam for String {
    fn as_param(&self) -> Param {
        Box::new(self.clone())
    }
}

impl AsParam for bool {
    fn as_param(&self) -> Param {
        Box::new(*self)
    }
}
