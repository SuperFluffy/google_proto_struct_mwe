#![allow(
    unreachable_pub,
    clippy::all,
    clippy::doc_markdown,
    clippy::similar_names,
    clippy::wildcard_imports
)]

#[allow(clippy::all)]
pub mod mypackage {
    include!(concat!(env!("OUT_DIR"), "/mypackage.rs"));
}

pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}
