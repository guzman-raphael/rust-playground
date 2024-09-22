use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Annotation {
    name: String,
    version: String,
    description: String,
}

impl Annotation {
    pub fn new(name: String, version: String, description: String) -> Self {
        let instance = Self {
            name,
            version,
            description,
        };
        let debug_display = format!("{:?}", instance);
        let name = debug_display.split("{").nth(0).unwrap();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("test_{}.yaml", name))
            .expect("Couldn't open file");

        serde_yaml::to_writer(file, &instance).unwrap();
        instance
    }
}

#[derive(Debug, Serialize)]
pub struct Pod {
    annotation: Annotation,
    command: String,
}

impl Pod {
    pub fn new(annotation: Annotation, command: String) -> Self {
        let instance = Self {
            annotation,
            command,
        };
        let debug_display = format!("{:?}", instance);
        let name = debug_display.split("{").nth(0).unwrap();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("test_{}.yaml", name))
            .expect("Couldn't open file");

        serde_yaml::to_writer(file, &instance).unwrap();
        instance
    }
}
