use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meaning_language::*;
use protos::{Protosizable, ReaderBudget, Textualizable};

fn budget() -> Budget {
    Budget { remaining: 4096, reader: ReaderBudget { remaining: 4096 }, depth: 0, maximum_depth: 4096 }
}

fn placeholder_link(name: &str) -> ContentLink {
    ContentLink { content_checksum: format!("unresolved-placeholder-{name}"), locator_option: None }
}

fn round_trip<T>(value: T) where T: datom_codec::Composing + Datomizable + Clone + std::fmt::Debug + PartialEq {
    let text = value.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<T>::from(text);
    assert_eq!(pending.actualize(&mut budget()).expect("generated Datom must parse"), value);
}

#[test]
fn lower_ontology_act_statement_and_annotation_round_trip() {
    let dimension = placeholder_link("visible-appearance");
    let quality = Quality {
        guna_kind: GunaKind::Rupa,
        quality_structure: QualityStructure::Domain(vec![DimensionDefinition { opaque_meaning: "visible appearance".into(), value_domain: ValueDomain::Categorical("unresolved palette".into()) }]),
        quality_coordinate_vector: vec![QualityCoordinate { dimension_link: dimension.clone(), quale: Quale::Text("blue".into()) }],
        bearer: Bearer { content_checksum: "unresolved-placeholder-bearer".into(), locator_option: None },
    };
    round_trip(Padartha::Guna(quality));

    let act = Act {
        dhatu: "jñā".into(),
        stem_composition: StemComposition::Single(StemFormation::San),
        lakara: Lakara::Lin(LinUse::Optative),
        prayoga: Prayoga::Kartari,
        purusha: Purusha::Prathama,
        vachana: Vachana::Eka,
        agreement_form_option: Some(AgreementForm { linga: Linga::Pum, linga_interpretation: LingaInterpretation::Pending }),
        karaka_vector: vec![Karaka { karaka_role: KarakaRole::Kartr, karaka_filler: KarakaFiller { content_checksum: "unresolved-placeholder-agent".into(), locator_option: None } }],
    };
    let statement = Statement { statement_content: StatementContent::ActPolicyPending(act), subpart_link_vector: vec![] };
    let canonical_statement = statement.clone().datomize(vec![]).protosize().textualize();
    println!("CANONICAL_STATEMENT={canonical_statement}");
    round_trip(statement);
    round_trip(Annotation { annotation_target: AnnotationTarget { content_checksum: "unresolved-placeholder-statement".into(), locator_option: None }, annotation_body: AnnotationBody::Prose("interpretation remains provisional".into()) });
}

#[test]
fn every_padartha_root_and_guna_kind_round_trips() {
    for root in [
        Padartha::Substance(Dravya::Prthivi),
        Padartha::Karman("restricted classical motion".into()),
        Padartha::Samanya("universal".into()),
        Padartha::Visesa("ultimate differentiator".into()),
        Padartha::Samavaya("inseparable inherence".into()),
        Padartha::Abhava("absence".into()),
    ] { round_trip(root); }
    for kind in [
        GunaKind::Rupa, GunaKind::Rasa, GunaKind::Gandha, GunaKind::Sparsa,
        GunaKind::Sankhya, GunaKind::Parimana, GunaKind::Prthaktva, GunaKind::Samyoga,
        GunaKind::Vibhaga, GunaKind::Paratva, GunaKind::Aparatva, GunaKind::Buddhi,
        GunaKind::Sukha, GunaKind::Duhkha, GunaKind::Iccha, GunaKind::Dvesa,
        GunaKind::Prayatna, GunaKind::Gurutva, GunaKind::Dravatva, GunaKind::Sneha,
        GunaKind::Dharma, GunaKind::Adharma, GunaKind::Sabda, GunaKind::Samskara,
    ] {
        round_trip(Padartha::Guna(Quality {
            guna_kind: kind,
            quality_structure: QualityStructure::Dimension(DimensionDefinition { opaque_meaning: "unresolved".into(), value_domain: ValueDomain::Unresolved("open".into()) }),
            quality_coordinate_vector: vec![], bearer: Bearer { content_checksum: "unresolved-placeholder-bearer".into(), locator_option: None },
        }));
    }
}

#[test]
fn malformed_arity_and_unknown_variant_are_rejected() {
    let mut missing_statement_field = Potential::<Statement>::from("{ prose None }");
    assert!(missing_statement_field.actualize(&mut budget()).is_err());
    let mut unknown_lakara = Potential::<Lakara>::from("FutureThatDoesNotExist");
    assert!(unknown_lakara.actualize(&mut budget()).is_err());
}
