# 2 — Understanding the Observable Set

Stage A (observe the whole) and Stage B (declare the structure). Content classes as in [Document 1](01_INTRODUCTION.md).

---

## 2.1 Stage A — the whole set  [EXPLANATORY]

**The arrangement.** A grooved ramp is propped on a table so that one end is higher than the other. Four pencil marks, P1 to P4, are drawn beside the groove from the high end to the low end, at uneven spacing. A marble released above P1 rolls down the groove past every mark.

**What is observed.**

1. *Values at the loci.* The height of each mark above the tabletop, read on a ruler held vertically on the table, in millimetres.
2. *Relations between the loci.* The marble is seen to roll from P1 directly to P2, from P2 directly to P3, and from P3 directly to P4. No other mark lies between consecutive marks, and the groove is one continuous track.

**The field (set 1, illustrative values):**

```
 height (mm)
  240 ┤ ● P1
      │   ╲  e0: P1 → P2
  180 ┤     ● P2
      │       ╲  e1: P2 → P3
  135 ┤         ● P3
      │           ╲  e2: P3 → P4
  110 ┤             ● P4
      └──────────────────────── along the ramp (not declared)
```

| Locus | Observable | Reading (set 1, illustrative) |
|---|---|---|
| P1 | height of mark P1 above the tabletop | 240.0 mm |
| P2 | height of mark P2 above the tabletop | 180.0 mm |
| P3 | height of mark P3 above the tabletop | 135.0 mm |
| P4 | height of mark P4 above the tabletop | 110.0 mm |

| Relation | Observational basis | Provenance class |
|---|---|---|
| e0: P1 → P2 | marble seen rolling from P1 directly to P2 along the one groove | Continuation |
| e1: P2 → P3 | marble seen rolling from P2 directly to P3 | Continuation |
| e2: P3 → P4 | marble seen rolling from P3 directly to P4 | Continuation |

The horizontal axis is drawn only so you can see the arrangement. Position along the ramp is **not** declared, and M discards it.

## 2.2 Establishing D and M  [CANONICAL rules, EXPLANATORY application]

**D, the observable extent.** Every reading is a number on the ruler's printed scale, 0 mm to 300 mm, inclusive. That interval is the declared extent. The IRRS gate refuses any reading outside it as OPEN, "recorded, not admitted". The V9 M Regional Instantiation Prerequisite asks for four things: a region, an observable extent, an orientation (what M preserves and discards), and a regional admission rule (`role_separation_and_operator_application.md` Step 1; `abr-irrs` README, The Regional M Prerequisite). The IRRS region must also be established from a recorded observable value inside the extent. The lesson uses the P1 reading for this (`observed_locus`).

**M, the measurement mapping.** M has kind *Identity*: each height reading enters D unchanged, in mm. A declared M states what it preserves and what it discards.

| M preserves | M discards |
|---|---|
| the height of each mark above the tabletop, in mm, as read on the ruler | position along the ramp; width of the pencil line; the marble's speed; reading uncertainty; everything else about the ramp |

[CANONICAL] "No transformation that alters pairwise differences precedes A or Δ unless declared within M" (`kernel_reference.md` §8). Identity alters nothing.

## 2.3 Establishing the topology  [CANONICAL rules, EXPLANATORY application]

The relational topology is **established by the observation**. It is not chosen for convenience and not taken from the order of a list.

**Each relation.** A relation is declared only where the observation shows one. Here that is the marble's passage from one mark directly to the next.

**Its direction.** "Every declared relation has exactly one admissible direction — the direction traceable to an observable through M" (§8). Here the direction is the marble's observed direction of travel: P1 → P2, not P2 → P1. The reverse would need its own independent observation.

**Its provenance.** "A relation with no declared provenance is fabricated within D" (§8). All three relations are *Continuation*: "within-family spatial continuation — declared from observable or device geometry. Open boundary: no wraparound" (§8). The one continuous groove is that geometry.

**What is not declared, and why.**

| Not declared | Reason |
|---|---|
| P1 → P3, P1 → P4, P2 → P4 | The marble's passage from P1 to P3 is observed only as passage through P2. Each declared relation is a direct passage. A longer relation would add a relation the observation does not separately establish. |
| P2 → P1 and other reverses | Not observed. "The reverse requires independent observable provenance; without it, it is inadmissible" (§8). |
| P4 → P1 | Not observed. It would close a ring: "Rings and tori are inadmissible" (§8). The track has an open boundary at both ends. |

**Interior loci.** P2 is the target of e0 and the source of e1. P3 is the target of e1 and the source of e2. Each has an observed incoming relation and an observed continuing relation. That is the full-operator admissibility condition [CANONICAL, §10]:

> A → B → R is admissible when M establishes at least one **interior locus** t: t = target(e) = source(f) for independently declared edges e and f, each with provenance through M. The minimal admissible interior is s → t → u. … M establishes it; the kernel does not compute it from `DeclaredRelations`.

So you **declare** the interior loci, with their basis. The program does not infer them. It only looks up the incoming and continuing relations you name in your declared relation list (Document 3, Step 4).

## 2.4 Why the set is complete within D  [EXPLANATORY]

- Every locus in D (P1–P4) has a reading.
- Every relation the observation establishes between those loci (three direct passages) is declared, each with its direction and provenance.
- No relation is declared that the observation does not establish.
- Two interior loci are established, so the applicable kernel is A → B → R, and every operator in it has what it needs.

This completeness is relative to the declared D. Outside D are the ramp's surface, the marble, the table, the room, and every property M discards. The lesson claims nothing about them.

## 2.5 The other declarations IRRS and the kernel require  [CANONICAL]

| Declaration | Why it is required | Set 1 (illustrative) |
|---|---|---|
| ρ_base | "M declares the operator parameters ρ_base and χ₀ … The kernel holds no value for any of them and supplies no default" (§5). R uses ρ | 0.5, *illustrative* |
| χ₀ | as above; "carries the dimension of χ" (§9), so mm here; must be finite and positive | 20.0 mm, *illustrative* |
| component pairs, cc | one component per locus, so no pairs are declared and cc is empty (§9) | none |
| detection tolerance for 𝟙[e] | the caller supplies it; "an IMPLEMENTATION-ONLY NUMERICAL TOLERANCE … never reported as" a property of the system (§11.5) | 1e-9 mm |
| observational support of each node, relation and interior locus | IRRS GR-5: "A declaration alone is not proof of an observed relationship." Observed requires evidence references | `proposed` (illustrative; nothing was observed) |
| persistence (Phase 2) | IRRS Continuous Relational Process Rule (`irrs-eval`); Phase 2 needs a declared sequence of actual observations (§8) | block present; prior evidence `open`; no sequence declared |

[OPEN] The values 0.5 and 20.0 mm were chosen by the Generator for this illustration only. They come from no other domain or repository, and they have no observational basis. They are returned to Origin (VERIFICATION_REPORT.md, Q-R2-3). ρ and R depend on them, so ρ and R in the supplied sets describe the illustrative parameters, not the ramp.

## 2.6 The declaration file  [EXPLANATORY]

The declaration is one JSON file in the **IRRS declaration schema** (`abr-irrs` `irrs-eval/src/declaration.rs`). IRRS reads exactly what you write. Three additions are read only by the lesson program, and IRRS ignores them:

- a `"class"` field on each edge (`"continuation"` or `"coupling"`);
- a `"lesson"` block, holding:
  - `observation_status` (`illustrative` or `measured`);
  - `m_kind` (`identity`);
  - `readings`;
  - `interior_loci` (each with `incoming`, `continuing` and `basis`);
  - `kernel_selection` (`abr`) and its basis;
  - `detection_tolerance`;
  - `comparisons`.

| Part of the file | What it declares | IRRS field |
|---|---|---|
| `inputs` | every value involved, with source, acquisition, limitations; illustrative readings are `simulated_input`, measured ones `primary_observable` | `Declaration.inputs` |
| `regional_prerequisite` | region, extent [0, 300] mm, orientation (preserves/discards), admission rule, `observed_locus` | `RegionalPrerequisiteDecl` |
| `mappings` | the authorized mapping `m.lesson1.identity` | `MappingDeclaration` |
| `abr_topology.nodes` | loci P1–P4, each bound to its observable | `NodeDeclaration` |
| `abr_topology.edges` | the three relations with their observational basis | `EdgeDeclaration` |
| `abr_topology.interior_locus` | one interior locus (the schema holds one; P2) | `InteriorLocusDeclaration` |
| `abr_topology.operator_parameters` | ρ_base, χ₀ with provenance `illustrative`, `declared` or `observational` | `OperatorParameterDeclaration` |
| `persistence` | the Phase 2 posture | `PersistenceDeclaration` |

Open [`declarations/set_1_ramp_illustrative.json`](declarations/set_1_ramp_illustrative.json) beside this page and find each item.

Next: [`03_CANONICAL_CALCULATION_GUIDE.md`](03_CANONICAL_CALCULATION_GUIDE.md).

*Bounded over D. No claim beyond D.*
