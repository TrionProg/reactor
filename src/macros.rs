

#[macro_export]
macro_rules! ok{
    () => {
        Ok(())
    };
    ($( $x:expr ),* ) => {
        Ok( $( $x, )* )
    }
} 
