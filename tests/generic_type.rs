use orcapod::generic_type::{Annotation, Pod};

#[test]
fn pod_correct() {
    let pod_name = String::from("test");
    let pod_version = String::from("0.1.0");
    let pod_description = String::from("This is a test example.");

    // let annotation = Annotation {
    //     name: pod_name,
    //     version: pod_version,
    //     description: pod_description,
    // };
    let annotation = Annotation::new(pod_name, pod_version, pod_description);
    // println!("{:#?}", annotation);
    // println!("{}", pod_name); // fails since Annotation moves ownership

    let pod_command = String::from("tail -f /dev/null");
    // let pod = Pod {
    //     annotation: annotation,
    //     command: pod_command,
    // };
    let pod = Pod::new(annotation, pod_command);
    println!("{:#?}", pod);
    // println!("{:#?}", annotation);  // fails since Pod moves ownership

    // assert_eq!(1, 4, "Raphael, it failed?!");
}
