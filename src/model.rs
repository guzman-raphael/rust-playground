use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Annotation {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct Pod {
    pub annotation: Annotation,
    pub source: String,
    pub image: String,
    pub command: String,
    pub min_memory: String,
    pub min_cpus: f32,
}

#[derive(Debug, Serialize)]
pub struct PodJob<'a> {
    pub annotation: Annotation,
    pub pod: &'a Pod,
    pub memory: String,
    pub cpus: f32,
}
