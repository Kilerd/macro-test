pub mod child;

#[macro_export]
macro_rules! impl_test {
    ($struct:ident) => {
        impl $struct {
            fn test() {
                println!("test");
            }
        }
    };
}