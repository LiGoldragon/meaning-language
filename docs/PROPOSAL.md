# Base meaning library proposal

Status: proposed and non-adopted. This library carries the ruled Vaiśeṣika
root vocabulary and the Pāṇinian Act surface; it does not settle the open
design decisions below.

`Padartha` is the seven padārthas. Its placement below a future
expressible-kind root is one reading, not a settled boundary. `Dravya` is its initial second-level payload,
with the nine classical substances. `Guna` is the initial twenty-four quality
vocabulary. `Karman` is intentionally an opaque, restricted classical-motion
payload. It is not the unrestricted grammatical `Act` type.

`Quality` separates its structure, declared dimension definitions, and
coordinates. A `QualityStructure` is a `Dimension` or a `Domain`; a domain
contains dimension definitions. Each coordinate links to a dimension definition
and carries a quale. A dimension definition declares a numeric, categorical,
or unresolved value domain. The decimal quale is one possible value form,
never a requirement for every guṇa. This is a partial structural borrowing from
UFO theory §3.12 (d5/a86–a99), not a claim that every guṇa is a gUFO Quality
or that this library enforces the theory's uniqueness, non-emptiness,
disjointness, or product constraints. `Samavaya` is modeled as its
own root and must not be read as identical to BFO `inheres in`.

Every completed `Statement`, `Annotation`, and subpart is referred to by
`ContentLink`. A link carries a checksum and optional index hint. An annotation
can therefore target a statement, an annotation, or any linked subpart.
`StatementContent` makes prose and Act alternative pending representations:
`ProsePolicyPending` retains the living's Twitter-style prose direction (with
an unruled exact word count), and `ActPolicyPending` carries a typed Act.
Neither is asserted as the permanent statement policy. `RetentionSpecification` is a proposal
contract only: no storage, hash, retention, GC, reference-count, or cycle
algorithm is implemented here.

Marked holes and parameters (not defaults):

1. the top-level expressible-kind shape and One/Many representation;
2. category roots versus subject-domain roots;
3. native skeleton versus imported vocabulary depth;
4. intention apart from certainty/evidentiality;
5. archive bytes versus canonical text as the checksum preimage;
6. whether `Linga` is grammatical only or receives additional semantics;
7. stem composition and whether its operator order is meaningful.

The `RetentionSpecification`, `StemComposition`, `LingaInterpretation`, and
`IntentionHole`
types are explicit pending descriptors, not settled policies. `StemComposition`
is deliberately opaque until (7) is ruled. No production hash is calculated
while (5) remains open.

`Lakara` has ten labels, including the Vedic `Let`. `LinUse` distinguishes optative from
benedictive presentation when applicable. `Linga` is optional and remains a
marked grammatical-agreement/form hole; `LingaInterpretation.Pending` makes
that unresolved status explicit and it does not encode a person's sex.
`StemComposition` may describe primary, `Nic` causative, `San` desiderative,
`Yan` intensive, and `Namadhatu` denominative formation, but makes no claim
about composition order. Gerundive/nonfinite kṛtya material is kept separate
from this finite-act surface. The desiderative is desire-to-perform, not a
typed assertion that every desiderative is intention.

`IntentionHole` is intentionally standalone: its final home (Act, guṇa, or a
different expression layer) remains with the living.

Ethos renders a bare enum member as a unit variant. Therefore the apparent
repeated heads `Karman.Karman`, `Samanya.Samanya`, `Visesa.Visesa`,
`Samavaya.Samavaya`, and `Abhava.Abhava` are retained only to carry their
opaque payload aliases; dropping the qualifier would silently sever the
payload. `Substance.Dravya` similarly carries the nine-dravya enum. This is a
current Ethos syntax constraint, not a naming decision.

The library validates structure only: Ethos-generated Datom values can be
parsed and reprinted. It does not validate a checksum preimage, resolve a
content link, enforce a value-domain coordinate, establish ontology truth, or
run a retention policy.
