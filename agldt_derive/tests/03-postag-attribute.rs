extern crate agldt_derive;
use agldt_derive::PostagFeature;

#[derive(PostagFeature)]
pub enum MockFeature {
    Masculine,
    #[postag('a')]
    Middle,
    Neuter,
}

fn main() {
    let a = MockFeature::Masculine;
    assert_eq!(format!("{}", a.to_agldt_postag()), "m".to_string());
    assert_eq!(a.to_agldt_postag().index(), 0);
}
