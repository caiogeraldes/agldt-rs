extern crate agldt;
use agldt::POSFeature;
use agldt::PostagFeature;
extern crate agldt_derive;
use agldt_derive::PostagFeature;

#[derive(PostagFeature)]
pub enum MockFeature {
    Masculine,
    Feminine,
    Neuter,
}

fn main() {}
