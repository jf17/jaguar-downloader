use std::fs::File;
use std::io::{Write};

struct Dependency {
    group_id: String,
    artifact_id: String,
    version: String,
}

impl Dependency {
    fn get_file_name(&self) -> String {
        format!("{}-{}.jar", self.artifact_id, self.version)
    }
    fn get_url(&self) -> String {
        format!("https://repo1.maven.org/maven2/{}/{}/{}/{}",
                self.group_id.replace(".", "/"),
                self.artifact_id,
                self.version,
                self.get_file_name())
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dependencies = vec![Dependency {
        group_id: "com.fifesoft".to_string(),
        artifact_id: "autocomplete".to_string(),
        version: "3.1.2".to_string(),
    }];

    for dep in &dependencies {
        let resp = reqwest::get(dep.get_url())
            .await?
            .bytes()
            .await?;

        File::create(dep.get_file_name())?
            .write_all(&resp).unwrap();
    }

    Ok(())
}