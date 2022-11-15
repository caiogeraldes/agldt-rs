extern crate agldt_derive;
use agldt_derive::PostagFeature;

#[derive(PostagFeature)]
pub enum MockFeature {
    Masculine,
    Middle,
    Neuter,
}

fn main() {}
