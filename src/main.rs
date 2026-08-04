use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
enum Loader {
    Fabric,
    Neoforge,
    Quilt,
    Forge,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
enum Version {
    Release(String),
    Candidate(String),
    Snapshot(String),
    Other(String),
}

impl Loader {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "fabric" => Some(Self::Fabric),
            "neoforge" => Some(Self::Neoforge),
            "quilt" => Some(Self::Quilt),
            "forge" => Some(Self::Forge),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    licenses: HashMap<String, i32>,
    versions: HashMap<String, i32>,
    loaders: HashMap<Loader, i32>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    project_id: String,
    project_type: String,
    all_project_types: Vec<String>,
    slug: String,
    author: Option<String>,
    author_id: Option<String>,
    organization: Option<String>,
    organization_id: Option<String>,
    title: String,
    description: String,
    categories: Vec<String>,
    display_categories: Vec<String>,
    versions: Vec<String>,
    downloads: i32,
    follows: i32,
    icon_url: String,
    date_created: String,
    date_modified: String,
    latest_version: String,
    license: String,
    client_side: String,
    server_side: String,
    gallery: Vec<String>,
    featured_gallery: Option<String>,
    color: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct ApiResult {
    hits: Vec<Project>,
    offset: i32,
    limit: i32,
    total_hits: i32,
}

fn main() {
    let mut data: Data = Data {
        licenses: HashMap::new(),
        versions: HashMap::new(),
        loaders: HashMap::new(),
    };

    let entries = std::fs::read_dir("results").unwrap();

    for i in entries {
        match i {
            Ok(entry) => {
                let mut file = File::open(entry.path()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                let v: ApiResult = serde_json::from_str(contents.as_str()).unwrap();

                for hit in v.hits {
                    let license = hit.license;
                    let versions = hit.versions;
                    let categories = hit.categories;

                    for category in categories {
                        match Loader::from_str(&category) {
                            Some(loader) => *data.loaders.entry(loader).or_insert(0) += 1,
                            None => (),
                        }
                    }

                    for version in versions {
                        *data.versions.entry(version).or_insert(0) += 1;
                    }

                    *data.licenses.entry(license).or_insert(0) += 1;
                }
            }
            Err(e) => println!("File open error \"{:?}\"", e),
        }
    }

    let mut file = File::create("data.json").unwrap();

    file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();

    let max_length = 90;

    let mut max_size = 0;

    for license in &data.versions {
        if *license.1 > max_size {
            max_size = *license.1;
        }
    }

    let divisor = (max_size as f32 / max_length as f32) as i32;
    let mut hash_vec: Vec<_> = data.versions.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    for i in hash_vec {
        let l = format!("{:?} ({})", i.0, i.1).len();
        println!(
            "{} ({}){}:  {}",
            i.0,
            i.1,
            " ".repeat(30 - l),
            "#".repeat((*i.1 / divisor) as usize + 1)
        );
    }
}
