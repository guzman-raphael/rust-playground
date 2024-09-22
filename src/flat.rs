#[derive(Debug)]
pub struct Annotation {
    name: String,
    version: String,
    description: String,
}

impl Annotation {
    pub fn new(name: String, version: String, description: String) -> Self {
        Self {
            name,
            version,
            description,
        }
    }
}

#[derive(Debug)]
pub struct Pod {
    annotation: Annotation,
    command: String,
}

impl Pod {
    pub fn new(annotation: Annotation, command: String) -> Self {
        Self {
            annotation,
            command,
        }
    }
}

// #[derive(Debug)]
// pub struct PodJob {
//     pub annotation: Annotation,
//     pub pod: Pod,
//     pub memory: String,
//     pub cpus: f32, // fix negative later
// }

// impl PodJob {
//     pub fn new(annotation: Annotation, pod: Pod, memory: String, cpus: f32) -> Self {
//         Self {
//             annotation,
//             pod,
//             memory,
//             cpus,
//         }
//     }
// }
