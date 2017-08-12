//! rancher-api is an interface that allows you to play with your Rancher servers in Rust.
#![deny(missing_docs,
        //missing_debug_implementations, 
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
extern crate serde_json;
extern crate curl;
extern crate regex;
extern crate base64;



/// Use Rancher module.
pub mod rancher;
/// Use Environment module.
pub mod environment;
/// Use Host module.
pub mod host;
/// Use Stack module.
pub mod stack;
/// Use Service module.
pub mod service;
/// Use Container module.
pub mod container;


pub use rancher::Rancher;
pub use environment::EnvironmentTrait;
pub use environment::Environment as Environment;
pub use host::HostTrait;
pub use host::Host as Host;
pub use stack::StackTrait;
pub use stack::Stack as Stack;
pub use service::ServiceTrait;
pub use service::Service as Service;
pub use container::ContainerTrait;
pub use container::Container as Container;
