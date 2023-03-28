use crate::param::Param;

pub trait AsParam {
    fn as_param(&self) -> Param;
}

macro_rules! impl_copy_as_param {
    ($t:ty) => {
        impl AsParam for $t {
            fn as_param(&self) -> Param {
                Box::new(*self)
            }
        }
    };
    ($($t:ty),*) => {
        $(impl_copy_as_param!($t);)*
    }
}

impl_copy_as_param![bool, u32, i32, i64];

macro_rules! impl_clone_as_param {
    ($t:ty) => {
        impl AsParam for $t {
            fn as_param(&self) -> Param {
                Box::new(self.clone())
            }
        }
    };
    ($($t:ty),*) => {
        $(impl_clone_as_param!($t);)*
    }
}

impl_clone_as_param![Vec<u8>, String];
