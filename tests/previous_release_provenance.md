# Provenance of the 0.2.0 texts in `previous_release.rs`

The expected datom texts in `tests/previous_release.rs` were produced by the 0.2.0 release itself:
origin main e4319d10567a54d493a2b4ec81f02030d4ccff5c (datom-codec 4dff16b, protos 15b41da,
ethos-zero c2653dd, aliases for the scalar declarations), exported with `git archive` into a scratch
directory, with this example added and run with `cargo run --features datom --example texts`. The
output was normalized to single-spaced text (the new test normalizes the same way).

```rust
use datom_codec::Datomizable;
use meaning_language::*;
use protos::{Protosizable, Textualizable};
fn t<T: Datomizable>(v: T) -> String { v.datomize(vec![]).protosize().textualize().split_whitespace().collect::<Vec<_>>().join(" ") }
fn main() {
    let link = |n: &str, l: Option<&str>| ContentLink { content_checksum: format!("cs-{n}"), locator_option: l.map(|x| x.to_string()) };
    println!("LINK1={}", t(link("a", None)));
    println!("LINK2={}", t(link("b", Some("loc"))));
    println!("KARMAN={}", t(Padartha::Karman("restricted classical motion".into())));
    println!("DOMAIN={}", t(QualityStructure::Domain(vec![DimensionDefinition { opaque_meaning: "visible appearance".into(), value_domain: ValueDomain::Categorical("palette".into()) }])));
    println!("QUALITY={}", t(Padartha::Guna(Quality { guna_kind: GunaKind::Rupa, quality_structure: QualityStructure::Dimension(DimensionDefinition { opaque_meaning: "u".into(), value_domain: ValueDomain::Unresolved("open".into()) }), quality_coordinate_vector: vec![QualityCoordinate { dimension_link: link("d", None), quale: Quale::Text("blue".into()) }], bearer: link("bearer", Some("l")) })));
    println!("ANNOT={}", t(Annotation { annotation_target: link("s", None), annotation_body: AnnotationBody::Prose("p".into()) }));
    println!("DHATU={}", t(Act { dhatu: "jñā".into(), stem_composition: StemComposition::Single(StemFormation::San), lakara: Lakara::Lin(LinUse::Optative), prayoga: Prayoga::Kartari, purusha: Purusha::Prathama, vachana: Vachana::Eka, agreement_form_option: None, karaka_vector: vec![Karaka { karaka_role: KarakaRole::Kartr, karaka_filler: link("agent", None) }] }));
}
```

Output, as pasted into the test:

```
LINK1={ cs-a None }
LINK2={ cs-b Some.loc }
KARMAN=Karman.«restricted classical motion»
DOMAIN=Domain.[ { «visible appearance» Categorical.palette } ]
QUALITY=Guna.{ Rupa Dimension.{ u Unresolved.open } [ { { cs-d None } Text.blue } ] { cs-bearer Some.l } }
ANNOT={ { cs-s None } Prose.p }
DHATU={ jñā Single.San Lin.Optative Kartari Prathama Eka None [ { Kartr { cs-agent None } } ] }
```

Limits: seven values, one per shape listed, not every type; the archive comparison in the same test
checks six of the eighteen newtypes against what they wrap. Both are sampled witnesses, not a proof that
no format changed.
