use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
enum Loader {
    Fabric,
    Neoforge,
    Quilt,
    Forge
}

impl Loader {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "fabric" => Some(Self::Fabric),
            "neoforge" => Some(Self::Neoforge),
            "quilt" => Some(Self::Quilt),
            "forge" => Some(Self::Forge),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    licenses: HashMap<String, i32>,
    versions: HashMap<String, i32>,
    loaders: HashMap<Loader, i32>
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
    color: Option<i32>
}

#[derive(Serialize, Deserialize)]
struct ApiResult {
    hits: Vec<Project>,
    offset: i32,
    limit: i32,
    total_hits: i32
}

fn main() {
    let mut data: Data = Data {
        licenses: HashMap::new(),
        versions: HashMap::new(),
        loaders: HashMap::new()
    };

    for i in 0..1491 {
        let mut file = File::open(format!("results/{}.json", i * 100)).unwrap();
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

    let mut file = File::create("data.json").unwrap();

    file.write_all(serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
}
