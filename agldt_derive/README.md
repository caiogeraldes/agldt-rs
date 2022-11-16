# agldt_derive

Procedural macro for deriving the implementation of the trait `agltd::PostagFeature` for enums.


 Derives the trait `agldt::PostagFeature` for an `Enum` corresponding to a feature.

 Suppose you have a feature `Gender`:

 ```rust
 enum Gender {
     Masculine,
     Feminine,
     Neuter,
 }
 ```

 Such feature has a representation in the postag string of the AGLDT XML notation, in the case
 of λελυμένων, the "m" at the sixth index postion:

 ```{xml}
 <word id="43" form="λελυμένων" lemma="λύω" postag="v-prpemg-" relation="ADV_CO" head="40"/>
 ```

 By declaring the enum as bellow, the method `to_agldt_postag()` becomes available:

 ```
 extern crate agldt_derive;
 extern crate agldt;
 use agldt::*;
 use agldt_derive::PostagFeature;

 #[derive(PostagFeature)]
 #[postagindex(6)]
 enum Gender {
     Masculine,
     Feminine,
     Neuter
 }

 let masc = Gender::Masculine;
 let masc_string = masc.to_agldt_postag().to_string();
 assert_eq!(masc_string, "m".to_string());
 assert_eq!(masc.to_agldt_postag().index(), 6);
 ```

 The derive chooses automatically a single character for representing the feature by using the
 first letter of the enum variant, but there might be issues: either two variants with the
 same first letter end up causing a conflict, or the user / standar requires non-trivial rules.
 In this case, the attribute `#[postag(<char>)] can be used:

 ```
 extern crate agldt;
 use agldt::POSFeature;
 use agldt::PostagFeature;
 extern crate agldt_derive;
 use agldt_derive::PostagFeature;

 #[derive(PostagFeature)]
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

 let perfect = TenseAspect::Perfect;
 let perfect_string = perfect.to_agldt_postag().to_string();
 assert_eq!(perfect_string, "r".to_string());
 assert_eq!(perfect.to_agldt_postag().index(), 3);
 ```

