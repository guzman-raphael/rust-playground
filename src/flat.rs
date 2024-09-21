#[derive(Debug)]
pub struct Annotation {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Pod {
    pub annotation: Annotation,
    pub command: String,
}
