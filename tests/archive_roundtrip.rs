//! Every generated type archives through rkyv, with or without the `datom`
//! feature (ethos-zero 16.0.0).
use meaning_language::*;

fn link(name: &str) -> ContentLink {
    ContentLink { content_checksum: format!("unresolved-placeholder-{name}"), locator_option: Some(format!("locator-{name}")) }
}

#[test]
fn a_quality_and_an_annotated_statement_return_from_the_archive_equal() {
    let quality = Padartha::Guna(Quality {
        guna_kind: GunaKind::Rupa,
        quality_structure: QualityStructure::Domain(vec![DimensionDefinition {
            opaque_meaning: "visible appearance".into(),
            value_domain: ValueDomain::Categorical("unresolved palette".into()),
        }]),
        quality_coordinate_vector: vec![QualityCoordinate { dimension_link: link("dimension"), quale: Quale::Whole(7) }],
        bearer: link("bearer"),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&quality).expect("archive Padartha");
    let back = rkyv::from_bytes::<Padartha, rkyv::rancor::Error>(&bytes).expect("read Padartha");
    assert_eq!(back, quality);

    let annotation = Annotation { annotation_target: link("statement"), annotation_body: AnnotationBody::LinkedStatement(link("linked")) };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&annotation).expect("archive Annotation");
    let back = rkyv::from_bytes::<Annotation, rkyv::rancor::Error>(&bytes).expect("read Annotation");
    assert_eq!(back, annotation);
}
