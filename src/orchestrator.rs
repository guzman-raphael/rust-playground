#[macro_export]
macro_rules! impl_DockerOrchestrator {
    ([$($Model:ty),+]) => {
        use std::process::Command;

        trait DockerOrchestrator: std::fmt::Debug {
            fn launch(&self);
        }

        $(impl DockerOrchestrator for $Model {
            fn launch(&self){
                let output = Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.annotation.name)
                    .arg("-d")
                    .arg(&self.pod.image)
                    .arg("sh")
                    .arg("-c")
                    .arg(&self.pod.command)
                    .output()
                    .expect("Failed to start pod job");
                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        })*
    }
}
