extern crate agldt;
use agldt::POSFeature;
use agldt::PostagFeature;
extern crate agldt_derive;
use agldt_derive::PostagFeature;

#[derive(PostagFeature)]
#[complexfeature(true)]
#[postagindex(3)]
pub enum TenseAspect {
    Future,
    #[postag('t')]
    FuturePerfect,
    Aorist,
    Imperfect,
    #[postag('r')]
    Perfect,
    Present,
    #[postag('l')]
    PlusPerfect,
    EMPTY,
}

fn main() {}
