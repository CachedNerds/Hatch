#![forbid(unsafe_code)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
pub extern crate clap;
#[macro_use]
pub extern crate failure;
pub extern crate git2;
pub extern crate os_info;
pub extern crate reqwest;
pub extern crate yaml_rust;

pub mod assets;
pub mod cli;
pub mod constants;
pub mod deps;
pub mod generators;
pub mod hatch_error;
pub mod locations;
pub mod platform;
pub mod project;
pub mod task;
pub mod yaml;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //    use serde_yaml;
        //
        //    #[derive(Debug, PartialEq, Serialize, Deserialize)]
        //    struct Length { a: f64 }
        //    #[derive(Debug, PartialEq, Serialize, Deserialize)]
        //    struct Point { x: f64, y: Length }
        //
        //    #[derive(Debug, PartialEq, Serialize, Deserialize)]
        //    struct Cfg {
        //      compiler: String,
        //    }
        //
        //    #[derive(Debug, PartialEq, Serialize, Deserialize)]
        //    struct CompiledProj {
        //      name: String,
        //      version: String,
        //      cfg: Cfg,
        //    }
        //
        //    #[derive(Debug, PartialEq, Serialize, Deserialize)]
        //    struct HeaderOnlyProj {
        //      name: String,
        //      version: String,
        //      cfg: Cfg,
        //    }
        //    // let proj = Proj { name: "foo".to_string(), version: "0.1.0".to_string(), cfg: Some(Cfg { compiler: "g++".to_string() }) };
        //    let proj = Proj { name: "foo".to_string(), version: "0.1.0".to_string(), cfg: None };
        //    let s = serde_yaml::to_string(&proj).unwrap();
        //    println!("{}", s);
        //    assert_eq!(s, "---\nx: 1\n\"y\": 2");
        //
        //    let deserialized_point: Point = serde_yaml::from_str(&s).unwrap();
        //    assert_eq!(point, deserialized_point);
    }
}
