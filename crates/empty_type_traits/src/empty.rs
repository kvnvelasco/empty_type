use crate::Container;
use std::error::Error;
use std::ops::{Deref, DerefMut};

pub struct Empty<E, F>(pub E, pub std::marker::PhantomData<F>);

impl<E, F> Empty<E, F>
where
    E: Container<Value = F>,
{
    pub fn resolve(mut self) -> F {
        match self.try_open() {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn try_resolve(mut self) -> Result<F, Box<dyn std::error::Error>> {
        self.try_open()
    }
}

impl<E, F> Deref for Empty<E, F> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E, F> DerefMut for Empty<E, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E, F> Container for Empty<E, F>
where
    E: Container<Value = F>,
{
    type Value = F;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        self.0.try_open()
    }
}
