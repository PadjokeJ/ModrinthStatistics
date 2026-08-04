use std::{collections::HashMap, fmt::Display};

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

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
enum Version {
    Release(String),
    Pre(String),
    Candidate(String),
    Snapshot(String),
    Other(String),
}

impl Version {
    fn from_str(s: &str) -> Self {
        if s[2..3] == *"w" {
            return Self::Snapshot(s.to_string());
        } else if s.contains("pre") {
            return Self::Pre(s.to_string());
        } else if s.contains("rc") {
            return Self::Candidate(s.to_string());
        } else if s.starts_with("1.") || s.starts_with("26.") {
            return Self::Release(s.to_string());
        } else {
            return Self::Other(s.to_string());
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    licenses: HashMap<String, i32>,
    versions: HashMap<Version, i32>,
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
                        *data.versions.entry(Version::from_str(&version)).or_insert(0) += 1;
                    }

                    *data.licenses.entry(license).or_insert(0) += 1;
                }
            }
            Err(e) => println!("File open error \"{:?}\"", e),
        }
    }

    let mut file = File::create("data.json").unwrap();

    /*file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();*/

    let max_length = 90;

    let mut max_size = 0;

    for i in &data.versions {
        match i.0 {
            Version::Snapshot(_) => (),
            _ => continue,
        };
        if *i.1 > max_size {
            max_size = *i.1;
        }
    }

    let divisor = (max_size as f32 / max_length as f32) as i32;
    let mut hash_vec: Vec<_> = data.versions.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    for i in hash_vec {
        match i.0 {
            Version::Snapshot(_) => (),
            _ => continue,
        };
        let l = format!("{:?} ({})", i.0, i.1).len();
        println!(
            "{:?} ({}){}:  {}",
            i.0,
            i.1,
            " ".repeat(40 - l),
            "#".repeat((*i.1 / divisor) as usize + 1)
        );
    }
}
