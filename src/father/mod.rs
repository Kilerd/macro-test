
#[macro_export]
macro_rules! impl_test {
($struct:ident) => {

        impl $struct {
            pub fn test() {
                println!("test");
            }
        }
    };
}

pub mod child;