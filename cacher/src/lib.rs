pub struct Cacher<F: Fn() -> R, R> {
    calculation: F,
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
            Some(ref v) => v,
            None => {
                let v = (self.calculation)();
                self.value = Some(v);
                self.value.as_ref().unwrap() // TODO design it better?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = Cacher::new(|| vec![0; 16]);
        assert_eq!(c.value(), &vec![0; 16]);
    }
}
