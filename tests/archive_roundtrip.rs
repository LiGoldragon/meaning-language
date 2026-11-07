//! Every generated type archives through rkyv, with or without the `datom`
//! feature (ethos-zero 16.0.0).
use meaning_language::*;

fn link(name: &str) -> ContentLink {
    ContentLink { content_checksum: ContentChecksum(format!("unresolved-placeholder-{name}")), locator_option: Some(Locator(format!("locator-{name}"))) }
}

#[test]
fn a_quality_and_an_annotated_statement_return_from_the_archive_equal() {
    let quality = Padartha::Guna(Quality {
        guna_kind: GunaKind::Rupa,
        quality_structure: QualityStructure::Domain(QualityDomain(vec![DimensionDefinition {
            opaque_meaning: OpaqueMeaning("visible appearance".into()),
            value_domain: ValueDomain::Categorical(OpaqueMeaning("unresolved palette".into())),
        }])),
        quality_coordinate_vector: vec![QualityCoordinate { dimension_link: DimensionLink(link("dimension")), quale: Quale::Whole(7) }],
        bearer: Bearer(link("bearer")),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&quality).expect("archive Padartha");
    let back = rkyv::from_bytes::<Padartha, rkyv::rancor::Error>(&bytes).expect("read Padartha");
    assert_eq!(back, quality);

    let annotation = Annotation { annotation_target: AnnotationTarget(link("statement")), annotation_body: AnnotationBody::LinkedStatement(link("linked")) };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&annotation).expect("archive Annotation");
    let back = rkyv::from_bytes::<Annotation, rkyv::rancor::Error>(&bytes).expect("read Annotation");
    assert_eq!(back, annotation);
}
