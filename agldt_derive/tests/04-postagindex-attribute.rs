extern crate agldt_derive;
use agldt_derive::PostagFeature;

#[derive(PostagFeature)]
#[postagindex(1)]
pub enum MockFeature {
    Hero,
    #[postag('a')]
    Human,
    Neuter,
}

fn main() {
    let a = MockFeature::Human;
    assert_eq!(format!("{}", a.to_agldt_postag()), "a".to_string());
    assert_eq!(a.to_agldt_postag().index(), 1);
}
