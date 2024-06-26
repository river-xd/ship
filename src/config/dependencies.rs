
use semver::VersionReq;
use std::collections::HashMap;

use serde::{
  Serialize,
  Deserialize
};




#[derive(Debug,Serialize,Deserialize,Default)]
pub struct Dependencies(HashMap<Box<str>,DependencyInfo>);


#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum DependencyInfo {
  Version(VersionReq),
  Table(DependencyTable)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct DependencyTable {
  version: Option<VersionReq>,
  #[serde(rename="git",alias="hg",alias="pijul",alias="fossil",alias="path")]
  path: Option<Box<str>>,
  branch: Option<Box<str>>,
  #[serde(default="_false")]
  optional: bool,
  registry: Option<Box<str>>,
  package: Option<Box<str>>,
}

const fn _false()-> bool { false }

impl Dependencies {
  pub async fn fetch(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn download(&self)-> anyhow::Result<()> {
    unimplemented!()
  }

  pub async fn build()-> anyhow::Result<()> {
    unimplemented!()
  }
}


