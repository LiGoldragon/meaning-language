//! What the 0.2.0 release wrote still reads the same: the datom texts below were produced by
//! origin main e4319d1 (0.2.0, scalar declarations as type aliases, datom-codec 4dff, protos
//! 15b4) from the equivalent values; the archives of the types that became newtypes are
//! compared with the archives of what they wrap.
use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meaning_language::*;
use protos::{Protosizable, ReaderBudget, Textualizable};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 4096,
    }
}

fn text<T: Datomizable>(value: T) -> String {
    value
        .datomize(vec![])
        .protosize()
        .textualize()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn link(name: &str, locator: Option<&str>) -> ContentLink {
    ContentLink {
        content_checksum: ContentChecksum(format!("cs-{name}")),
        locator_option: locator.map(|locator| Locator(locator.to_string())),
    }
}

fn meaning(words: &str) -> OpaqueMeaning {
    OpaqueMeaning(words.to_string())
}

fn assert_reads_back<T>(expected: T, earlier_text: &str)
where
    T: datom_codec::Composing + Datomizable + Clone + std::fmt::Debug + PartialEq,
{
    assert_eq!(text(expected.clone()), earlier_text);
    let mut pending = Potential::<T>::from(earlier_text.to_string());
    assert_eq!(
        pending.actualize(&mut budget()).expect("the 0.2.0 text reads"),
        expected
    );
}

#[test]
fn the_texts_of_0_2_0_read_equal_and_write_the_same() {
    assert_reads_back(link("a", None), "{ cs-a None }");
    assert_reads_back(link("b", Some("loc")), "{ cs-b Some.loc }");
    assert_reads_back(
        Padartha::Karman(Karman(meaning("restricted classical motion"))),
        "Karman.«restricted classical motion»",
    );
    assert_reads_back(
        QualityStructure::Domain(QualityDomain(vec![DimensionDefinition {
            opaque_meaning: meaning("visible appearance"),
            value_domain: ValueDomain::Categorical(meaning("palette")),
        }])),
        "Domain.[ { «visible appearance» Categorical.palette } ]",
    );
    assert_reads_back(
        Padartha::Guna(Quality {
            guna_kind: GunaKind::Rupa,
            quality_structure: QualityStructure::Dimension(DimensionDefinition {
                opaque_meaning: meaning("u"),
                value_domain: ValueDomain::Unresolved(meaning("open")),
            }),
            quality_coordinate_vector: vec![QualityCoordinate {
                dimension_link: DimensionLink(link("d", None)),
                quale: Quale::Text(meaning("blue")),
            }],
            bearer: Bearer(link("bearer", Some("l"))),
        }),
        "Guna.{ Rupa Dimension.{ u Unresolved.open } [ { { cs-d None } Text.blue } ] { cs-bearer Some.l } }",
    );
    assert_reads_back(
        Annotation {
            annotation_target: AnnotationTarget(link("s", None)),
            annotation_body: AnnotationBody::Prose(meaning("p")),
        },
        "{ { cs-s None } Prose.p }",
    );
    assert_reads_back(
        Act {
            dhatu: Dhatu("jñā".to_string()),
            stem_composition: StemComposition::Single(StemFormation::San),
            lakara: Lakara::Lin(LinUse::Optative),
            prayoga: Prayoga::Kartari,
            purusha: Purusha::Prathama,
            vachana: Vachana::Eka,
            agreement_form_option: None,
            karaka_vector: vec![Karaka {
                karaka_role: KarakaRole::Kartr,
                karaka_filler: KarakaFiller(link("agent", None)),
            }],
        },
        "{ jñā Single.San Lin.Optative Kartari Prathama Eka None [ { Kartr { cs-agent None } } ] }",
    );
}

fn archived<T>(value: &T) -> Vec<u8>
where
    T: for<'a> rkyv::Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error,
        >,
    >,
{
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .expect("archives")
        .to_vec()
}

#[test]
fn a_newtype_archives_to_the_bytes_of_what_it_wraps() {
    assert_eq!(archived(&meaning("visible")), archived(&"visible".to_string()));
    assert_eq!(archived(&Dhatu("jñā".into())), archived(&"jñā".to_string()));
    assert_eq!(
        archived(&ContentChecksum("cs".into())),
        archived(&"cs".to_string())
    );
    let wrapped = link("a", Some("loc"));
    assert_eq!(archived(&Bearer(wrapped.clone())), archived(&wrapped));
    assert_eq!(archived(&KarakaFiller(wrapped.clone())), archived(&wrapped));
    let dimensions = vec![DimensionDefinition {
        opaque_meaning: meaning("visible appearance"),
        value_domain: ValueDomain::Categorical(meaning("palette")),
    }];
    assert_eq!(
        archived(&QualityDomain(dimensions.clone())),
        archived(&dimensions)
    );
}
