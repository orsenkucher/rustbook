pub struct Cacher<T, R>
where
    T: Fn() -> R,
{
    calculation: T,
    value: Option<R>,
}

impl<T, R> Cacher<T, R>
where
    T: Fn() -> R,
{
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self) -> &R {
        match self.value {
            Some(v) => &v,
            None => {
                let v = (self.calculation)();
                self.value = Some(v);
                &self.value.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
