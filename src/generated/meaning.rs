#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type OpaqueMeaning = String;
#[rustfmt::skip]
pub type ContentChecksum = String;
#[rustfmt::skip]
pub type Locator = String;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContentLink {
    pub content_checksum: ContentChecksum,
    pub locator_option: Option<Locator>,
}
#[rustfmt::skip]
pub type Bearer = ContentLink;
#[rustfmt::skip]
pub type AnnotationTarget = ContentLink;
#[rustfmt::skip]
pub type DimensionLink = ContentLink;
#[rustfmt::skip]
pub type KarakaFiller = ContentLink;
#[rustfmt::skip]
pub type SubpartLink = ContentLink;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Padartha {
    Dravya(Dravya),
    Guna(Quality),
    Karman(Karman),
    Samanya(Samanya),
    Visesa(Visesa),
    Samavaya(Samavaya),
    Abhava(Abhava),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dravya {
    Prthivi,
    Ap,
    Tejas,
    Vayu,
    Akasa,
    Kala,
    Dik,
    Atman,
    Manas,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GunaKind {
    Rupa,
    Rasa,
    Gandha,
    Sparsa,
    Sankhya,
    Parimana,
    Prthaktva,
    Samyoga,
    Vibhaga,
    Paratva,
    Aparatva,
    Buddhi,
    Sukha,
    Duhkha,
    Iccha,
    Dvesa,
    Prayatna,
    Gurutva,
    Dravatva,
    Sneha,
    Dharma,
    Adharma,
    Sabda,
    Samskara,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Karman {
    pub opaque_meaning: OpaqueMeaning,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Samanya {
    pub opaque_meaning: OpaqueMeaning,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Visesa {
    pub opaque_meaning: OpaqueMeaning,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Samavaya {
    pub opaque_meaning: OpaqueMeaning,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Abhava {
    pub opaque_meaning: OpaqueMeaning,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValueDomain {
    Numeric(OpaqueMeaning),
    Categorical(OpaqueMeaning),
    Unresolved(OpaqueMeaning),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DimensionDefinition {
    pub opaque_meaning: OpaqueMeaning,
    pub value_domain: ValueDomain,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QualityDomain {
    pub dimension_definition_vector: std::vec::Vec<DimensionDefinition>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum QualityStructure {
    Dimension(DimensionDefinition),
    Domain(QualityDomain),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Quale {
    Text(OpaqueMeaning),
    Whole(i64),
    Decimal(datom_codec::Decimal),
    Referenced(ContentLink),
    Unresolved(OpaqueMeaning),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QualityCoordinate {
    pub dimension_link: DimensionLink,
    pub quale: Quale,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quality {
    pub guna_kind: GunaKind,
    pub quality_structure: QualityStructure,
    pub quality_coordinate_vector: std::vec::Vec<QualityCoordinate>,
    pub bearer_option: Option<Bearer>,
}
#[rustfmt::skip]
pub type Dhatu = String;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StemFormation {
    Nic,
    San,
    Yan,
    Namadhatu,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StemComposition {
    Primary,
    Single(StemFormation),
    CompositionPending(OpaqueMeaning),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Lakara {
    Lat,
    Lit,
    Lut,
    Lrt,
    Let,
    Lot,
    Lan,
    Lin(LinUse),
    Lun,
    Lrn,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LinUse {
    Optative,
    Benedictive,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prayoga {
    Kartari,
    Karmani,
    Bhave,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vachana {
    Eka,
    Dvi,
    Bahu,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Linga {
    Pum,
    Stri,
    Napumsaka,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LingaInterpretation {
    Pending,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AgreementForm {
    pub linga: Linga,
    pub linga_interpretation: LingaInterpretation,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KarakaRole {
    Kartr,
    KarmanKaraka,
    Karana,
    Sampradana,
    Apadana,
    Adhikarana,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Karaka {
    pub karaka_role: KarakaRole,
    pub karaka_filler: KarakaFiller,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Act {
    pub dhatu: Dhatu,
    pub stem_composition: StemComposition,
    pub lakara: Lakara,
    pub prayoga: Prayoga,
    pub purusha: Purusha,
    pub vachana: Vachana,
    pub agreement_form_option: Option<AgreementForm>,
    pub karaka_vector: std::vec::Vec<Karaka>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Statement {
    pub opaque_meaning: OpaqueMeaning,
    pub act_option: Option<Act>,
    pub subpart_link_vector: std::vec::Vec<SubpartLink>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AnnotationBody {
    Prose(OpaqueMeaning),
    Statement(ContentLink),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Annotation {
    pub annotation_target: AnnotationTarget,
    pub annotation_body: AnnotationBody,
}
#[rustfmt::skip]
pub type RootStatement = SubpartLink;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RetentionSpecification {
    pub opaque_meaning: OpaqueMeaning,
}
