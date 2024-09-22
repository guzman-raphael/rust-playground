// extern crate orcapod;
use orcapod::flat::{Annotation, Pod};

fn main() {
    let pod_name = String::from("test");
    let pod_version = String::from("0.1.0");
    let pod_description = String::from("This is a test example.");

    // let annotation = Annotation {
    //     name: pod_name,
    //     version: pod_version,
    //     description: pod_description,
    // };

    // // println!("{:#?}", annotation);
    // // println!("{}", pod_name); // fails since Annotation moves ownership

    // let pod_command = String::from("tail -f /dev/null");
    // let pod = Pod {
    //     annotation: annotation,
    //     command: pod_command,
    // };
    // println!("{:#?}", pod);
    // // println!("{:#?}", annotation);  // fails since Pod moves ownership
}
