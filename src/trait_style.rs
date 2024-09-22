// pub trait Storage: std::fmt::Debug {
//     // fn store(&self<T>) {
//     //     println!("Storing something");
//     // }
//     fn delete(&self) {
//         println!("Deleting : {:#?}", self);
//         // println!("Command was: {}", self.command);
//         println!("Done!");
//     }
//     // fn load(&self<T>) {
//     //     println!("loading something");
//     // }
// }

// #[derive(Debug)]
// pub struct Pod {
//     pub command: String,
// }

// impl Storage for Pod {}

#[macro_export]
macro_rules! impl_Storage {
    ([$($t:ty),+]) => {
        trait Storage: std::fmt::Debug {
            fn delete(&self);
        }
        $(impl Storage for $t {
            fn delete(&self) {
                println!("Deleting : {:#?}", self);
                println!("Command was: {}", self.command);
                println!("Done!");
            }
        })*
    }
}
// pub(crate) use impl_Storage;

#[derive(Debug)]
pub struct Pod {
    pub command: String,
}

// impl_Storage! {[Pod]}
