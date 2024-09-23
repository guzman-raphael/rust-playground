use orcapod::model::{Annotation, Pod, PodJob};
use orcapod::{impl_DockerOrchestrator, impl_FileStorage};

#[test]
fn verify_composition() {
    let pod = Pod {
        annotation: Annotation {
            name: String::from("kilosort"),
            version: String::from("4.0.15"),
            description: String::from("This is a kilosort pod."),
        },
        source: String::from("https://github.com/MouseLand/Kilosort/tree/v4.0.15"),
        image: String::from("alpine:latest"),
        command: String::from("tail -f /dev/null"),
        min_memory: String::from("2GiB"),
        min_cpus: 0.25,
    };

    let pod_job = PodJob {
        annotation: Annotation {
            name: String::from("kilosort"),
            version: String::from("0.1.0"),
            description: String::from("This is a kilosort pod job."),
        },
        pod: &pod,
        memory: String::from("16GiB"),
        cpus: 2.0,
    };
    // println!("{:#?}", pod);
    // println!("{:#?}", pod_job);
    impl_DockerOrchestrator! {[PodJob<'_>]}; // would prefer to avoid annonymous lifetime
    impl_FileStorage! {[Pod, PodJob<'_>]}; // would prefer to avoid annonymous lifetime
    pod_job.launch();
    pod.store();
    pod_job.store();
}

#[test]
fn another_one() {
    println!("yo");
    println!("bye");
    assert_eq!(1, 2, "Raphael, it failed?!");
}
