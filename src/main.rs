use std::fs::File;
use std::io::{Write};
use curl::easy::Easy;

struct Dependency {
    repo: String,
    group_id: String,
    artifact_id: String,
    version: String,
}

impl Dependency {
    fn get_file_name(&self) -> String {
        format!("{}-{}.jar", self.artifact_id, self.version)
    }
    fn get_url(&self) -> String {
        format!("{}/{}/{}/{}/{}",
                self.repo,
                self.group_id.replace(".", "/"),
                self.artifact_id,
                self.version,
                self.get_file_name())
    }
}

fn main() {
    let maven_repo = "https://repo1.maven.org/maven2";

    let dependencies = vec![Dependency {
        repo: maven_repo.to_string(),
        group_id: "com.fifesoft".to_string(),
        artifact_id: "autocomplete".to_string(),
        version: "3.1.2".to_string(),
    }];
    for dep in &dependencies {
        let mut data = Vec::new();
        let mut handle = Easy::new();
        handle.url(&dep.get_url()).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        File::create(dep.get_file_name()).unwrap()
            .write_all(&data).unwrap();
    }
}