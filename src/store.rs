#[macro_export]
macro_rules! impl_FileStorage {
    ([$($Model:ty),+]) => {
        use std::fs;

        trait Storage: std::fmt::Debug {
            fn store(&self);
        }

        $(impl Storage for $Model {
            fn store(&self){
                let mut yaml = serde_yaml::to_string(&self).unwrap();  // kinda unsafe, better to try with ?

                let debug_display = format!("{:?}", self);
                let class = debug_display.split(" {").nth(0).unwrap(); // kinda unsafe, better to try with ?
                yaml.push_str(&format!("class: {}\n", class));  // would be nice to use writeln! somehow since it adds newline

                fs::write(
                    format!("test_store/test_{}_{}.yaml", class, self.annotation.name),
                    yaml
                ).expect("Unable to write file");
            }
        })*
    }
}
