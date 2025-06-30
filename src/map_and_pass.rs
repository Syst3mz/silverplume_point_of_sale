use anyhow::Error;

pub trait MapAndPass<T> {
    /// Applies `func` to the value contained in `self` If `self` is `Ok` and maps the type of the
    /// `self` to `Result((), Error)  
    fn map_and_pass(self, func: impl FnMut(T)) -> Result<(), Error>;
}

impl<T> MapAndPass<T> for Result<T, Error> {
    fn map_and_pass(self, mut func: impl FnMut(T)) -> Result<(), Error> {
        match self {
            Ok(v) => {
                func(v);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}