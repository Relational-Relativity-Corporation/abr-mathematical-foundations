# Lesson 1 — Verification Report (revision 2)

**Prepared by:** Generator (AI), 2026-11-05. **For:** Origin and an independent Verifier.
**Status:** submitted for review. The Generator reports execution results and does not declare verification complete.

Everything below was observed during execution on 2026-11-05: Windows 11 Home 10.0.26200, `cargo 1.97.1`, `rustc 1.97.1`. Nothing has been committed or pushed. The revision-1 report is preserved as [`VERIFICATION_REPORT_R1.md`](VERIFICATION_REPORT_R1.md).

---

## 1. What changed in revision 2

| | Revision 1 | Revision 2 |
|---|---|---|
| Principal structure | two loci, one relation (Primary kernel Δ) | **complete bounded observable set**: four loci, three relations, two interior loci (ABR kernel A → B → R) |
| Principal program | `src/main.rs` (`lesson01`) | `src/bin/observable_set.rs` (`observable_set`, default) |
| IRRS pathway | `relational-gate` admission + V9 `admit` | `irrs-eval` declaration evaluation + `relational-gate` admission + `irrs-session` execution |
| Two-locus exercise | principal | supporting (Document 5, Exercise 4); source unchanged |

## 2. Evidence table

| Requirement | Evidence | Status |
|---|---|---|
| **Canonical authority** | `abr-kernel` `1c1ef70a69b3175a2df0458d0ebf1f71475a3d62`; `abr-irrs` `c7410c590d04a96612619e639afdce7f0751aeca` (packages `relational-gate`, `irrs-eval`, `irrs-session`, `irrs-kernel`). Pinned by `rev` in `Cargo.toml`, resolved in `Cargo.lock` (sha256 `738de7dc…7a70`) and printed in every report | — |
| **Observable set** | `declarations/set_1_ramp_illustrative.json`. D: heights on a ruler scale [0, 300] mm inclusive. M: Identity, with what it preserves and discards. Loci P1–P4. Relations P1→P2, P2→P3, P3→P4, Continuation, each with an observational basis (marble's observed passage). Interior loci P2, P3 declared with their incoming and continuing relations. ρ_base 0.5, χ₀ 20.0 mm (illustrative), tolerance 1e-9 mm | DECLARED (illustrative) |
| **IRRS declaration evaluation** | `irrs_eval::Evaluator::evaluate` → status `Open`, 14 OPEN conditions, 0 violations: 4 Phase 2, 4 nodes `proposed`, 3 edges `proposed`, interior locus `proposed`, 2 illustrative parameters. Full record: `records/set_1_ramp_illustrative.irrs_eval.json` | OPEN (not authorized) |
| **IRRS observation admission** | `RegionBuilder::establish(240.0)` → region established; `AdmissionGate::submit` → P1 240.0, P2 180.0, P3 135.0, P4 110.0 mm, each ADMITTED (raw = gate-computed) | ADMITTED |
| **Kernel selection** | declared `abr`; P2 and P3 found as declared target/source of declared relations (`kernel_reference.md` §10) | ABR kernel |
| **Execution** | `irrs_session::execute::execute` → `operator_a`, `compute_rho`, `operator_b`, `operator_r`, `detection_indicator`; independent re-execution (`verify_abr_irrs`) agreed | CALCULATED (§4) |
| **V9 provenance admission** | not available for A → B → R: `abr-kernel` `src/admission.rs` supported scope, "OPEN: ABR kernel A → B → R" | OPEN |
| **Second observation set** | `declarations/set_2_ramp_lowered_illustrative.json` → same IRRS status (14 OPEN, 0 violations), all readings admitted, CALCULATED (§4) | CALCULATED |
| **Missing declaration** | `declarations/set_3_parameters_undeclared_illustrative.json` → IRRS OPEN (13 conditions), all readings admitted, run stopped at Step 5: ρ_base, χ₀ not declared, nothing executed | OPEN (correct stop) |
| **Primary condition (supporting)** | `declarations/experiment_5_ramp_segment_illustrative.txt` via `lesson01` → IRRS gate ADMITTED both; `execute_primary` → `operator_delta`; V9 `admit` ADMITTED, status `Synthetic`; Δ[P2 → P3] = 45.0 mm; Σ NotEvaluated (ρ_base, χ₀) | CALCULATED |
| **Comparison with observation** | every supplied set is illustrative; every comparison is `not_compared` | OPEN (demonstration only) |
| **Primary Region** | not observed and not reconstructed; boundary documented (Document 5) | OPEN |
| **Revision-1 evidence** | `src/main.rs` byte-identical to revision 1; `records/experiment_1..4` regenerate byte-identical | Preserved |

## 3. Canonical baseline (re-run for revision 2)

- **Kernel** (`abr-kernel` `1c1ef70`, clean checkout), `cargo test`: 61 lib, 3 `provenance_demo`, 12 `v9_admission`, 35 `v9_conformance` and 1 doctest passed. **112 passed, 0 failed.**
- **IRRS** (`abr-irrs` `c7410c5`, clean checkout):
  - `ABR_KERNEL_DIR=<kernel checkout> cargo test --workspace` **does not compile** (exit 101). `examples/irrs-e2e-tests` has 10 errors: E0432 unresolved imports `irrs_session::admission::{DeclaredObservable, …}`, `irrs_session::admission::{AdmissionResult, SessionOutput, ExecutionResult}` and `irrs_session::session`; E0433 `OpenCondition` ×5 and `Violation` ×2. This is unchanged from revision 1 (finding Q-6). IRRS was not modified.
  - With that package excluded: **189 passed, 0 failed.** The test targets include irrs-session `execute_tests` (18) and irrs-eval (36).
  - `irrs-eval verify-kernel --kernel-dir <kernel checkout>` exit 0: "V9 conformance complete … at 1c1ef70a… (Origin-authorized); 10 canonical files equal." This is the basis for treating `irrs-kernel`'s operators as the canonical ones.
- **Lesson**: `cargo build` finished with no warnings and no errors.
- **Clean reproduction**: the lesson files (no `target/`, no `records/`) were copied to an empty directory and built `--locked` with an empty `CARGO_HOME`, which forced a fresh fetch from GitHub and crates.io. The reports and IRRS records of sets 1–3, and the experiment 5 report, were **byte-identical** to those in `records/`.

## 4. Canonical operator outputs (actual; illustrative inputs)

Inputs to `irrs_session::execute`:

- NodeField x[0] = gate-admitted readings, in node order P1–P4;
- DeclaredRelations edges [(0,1), (1,2), (2,3)], all Continuation;
- ρ_base 0.5, χ₀ 20.0 mm;
- pairs [] and cc [];
- tolerance 1e-9 mm.

**Set 1** (readings 240.0, 180.0, 135.0, 110.0 mm):

| Relation | A | B | R | 𝟙 |
|---|---|---|---|---|
| P1 → P2 | 60.0 | 105.0 | 131.25 | 1.0 |
| P2 → P3 | 45.0 | 70.0 | 40.0 | 1.0 |
| P3 → P4 | 25.0 | 25.0 | 0.76923076923077 | 1.0 |

ρ: P1 0.375, P2 0.375, P3 0.34615384615384615, P4 0.2777777777777778.

**Set 2** (readings 210.0, 162.0, 126.0, 110.0 mm):

| Relation | A | B | R | 𝟙 |
|---|---|---|---|---|
| P1 → P2 | 48.0 | 84.0 | 102.3529411764706 | 1.0 |
| P2 → P3 | 36.0 | 52.0 | 28.0 | 1.0 |
| P3 → P4 | 16.0 | 16.0 | −0.7142857142857153 | 1.0 |

ρ: P1 0.35294117647058826, P2 0.35294117647058826, P3 0.32142857142857145, P4 0.2222222222222222.

**Differences between the sets.** Topology, kernel selection and IRRS status are identical. Every A and B is smaller in set 2. R[P3 → P4] changes sign. 𝟙 = 1.0 on every relation in both. The persistence operators were not applied: the two sets are separate declarations, not a declared process sequence (§8).

Both runs surfaced the kernel open conditions ChiMaxSelectionRule (OPEN), RhoSaturatingForm (OPEN), EmptyAdjacency A-3 (OPEN) and DetectionIndicatorAudit (HOLD). R on P1 → P2 (no predecessor) and on P3 → P4 (no successor) contains a sum over an empty declared set, so it depends on A-3. `relational_isolation(rel)` = false in both sets.

**Generator cross-check (not lesson evidence).** The Generator substituted the outputs above by hand into the forms in `kernel_reference.md` §11.1 and found them consistent. This is not part of the lesson and is not a substitute for the canonical execution.

## 5. Pathways identified

| Stage | Canonical code | Why |
|---|---|---|
| Declaration evaluation | `irrs_eval::Evaluator::evaluate` (`irrs-eval/src/evaluator.rs`, Stage 2 ABR topology admission) | The IRRS evaluator for D, M, region, mappings, ABR nodes, edges, interior locus and parameters. The student's file *is* an `irrs-eval` declaration, and the digest is computed as the CLI computes it |
| Observation admission | `relational_gate::{RegionBuilder, AdmissionGate}` | The only constructor of `AdmittedObservation<T>`. Only `gate_result()` values enter the NodeField |
| Execution | `irrs_session::execute::execute` | The IRRS execution interface for A → B → R ("Origin declares → IRRS calls canonical operators → IRRS returns results"), with RPPI re-execution |
| V9 admission | `abr_kernel::admission::admit` | Used only in the supporting Primary exercise. ABR is outside its canonical scope |

## 6. Self-audit (instruction §9)

| Check | Result |
|---|---|
| The lesson begins with a complete bounded observable set | **Yes.** Documents 1–2 and set 1: four loci, three relations, completeness stated relative to D (Document 2 §2.4) |
| Topology justified by the observation | **Yes, as declared.** Each relation is the marble's observed direct passage; direction is the observed travel; undeclared relations and the reasons are listed (Document 2 §2.3). In the supplied sets the observation is illustrative, so IRRS holds every element `proposed` (OPEN) |
| All calculations use canonical implementations | **Yes.** Every A, ρ, B, R and 𝟙 value is an `irrs_session::execute` return value; Δ is an `execute_primary` return value. The program performs no arithmetic on readings or outputs |
| IRRS admission actually occurs | **Yes.** `irrs-eval` evaluation and `relational-gate` admission run on every set, and their outcomes are printed and preserved |
| Applicable operators execute or report canonical limitations | **Yes.** A → B → R executed when declarations were complete. Set 3 stopped at the missing ρ_base and χ₀. Δ executed for the single relation. V9 admission of ABR is reported OPEN. Phase 2 is not applicable |
| No mathematical definitions introduced | **Yes, with one item for review.** Step 4 checks each declared interior locus by looking up its named incoming and continuing relations in the declared edge list (§7 item 3). It is a membership lookup implementing the §10 wording, not an inference of topology |
| Student can reproduce | **Yes.** Clean reproduction byte-identical (§3); measured template and procedure in Document 4, Exercise 5 |
| Primary Region not claimed observable | **Yes.** Document 5 states that the Primary Region is neither observed nor reconstructed. The ramp reaches only "the Primary condition", and the identification is OPEN |
| Verification distinguished from calculation | **Yes.** Separate status lines; illustrative sets print "OPEN — DEMONSTRATION ONLY" |
| Unresolved conditions marked OPEN | **Yes.** §8 and the documents |

## 7. What the lesson's own code does (for Verifier review)

`src/bin/observable_set.rs` contains no operator, admission rule or comparison rule. Review the following:

1. It parses one JSON file twice: as `irrs_eval::Declaration` (unknown fields ignored by its serde derive) and as a JSON value for the `lesson` block and each edge's `class`.
2. It supplies the declared Identity M to the gate as `AuthorizedMapping` closures: `|p, _| Ok(p)` and `|v| *v`. Only `m_kind = identity` is accepted.
3. **Step 4 lookup.** For each declared interior locus, the named incoming relation must be in the declared edge list with target = the node, and the named continuing relation must be in the list with source = the node. If none passes, the program reports the Primary condition and executes nothing. If a passing locus exists and the declared selection is not `abr`, it reports REJECTED.
4. **Step 5 completeness.** ρ_base and χ₀ must be present in `operator_parameters`, the tolerance in `lesson`, and every edge needs class `continuation` or `coupling`. Otherwise the program stops and names each missing item.
5. **Execution with IRRS OPEN.** The program executes when IRRS returns OPEN with no violations, and stops on REJECTED (Q-R2-2).
6. Node index is the list position in `abr_topology.nodes`, and the report states that it is a label only.
7. The status lines are derived by matching declared strings and outcome variants. VERIFIED requires `measured`, all four comparison fields, and `agrees`. The VERIFIED and DIVERGENT branches were **not executed**, because no measured set exists; writing one would fabricate an observation.
8. `Box::leak` supplies the `&'static str` labels the gate API requires.

## 8. Generator checks on decision branches (outputs discarded; not lesson evidence)

Variants of set 1 were run from the scratchpad and their records were deleted afterwards:

| Variant | Observed outcome |
|---|---|
| `kernel_selection = primary` with interior loci declared | Kernel selection REJECTED; nothing executed |
| interior locus naming an undeclared incoming relation | not established → Primary condition reported; nothing executed |
| one edge without `class` | Step 5 stop: provenance class missing |
| P1 = 350 mm | gate: "OPEN: value 350 outside declared extent [0, 300] — recorded, not admitted"; stop |
| edge to undeclared node P9 | IRRS REJECTED ("to_node 'P9' not in the declared node inventory … GR-4"); stop |
| `interior_loci` empty | Primary condition reported; nothing executed |
| closing edge P4 → P1 (ring) | **IRRS OPEN with 0 violations, gate ADMITTED, A → B → R executed over 4 relations** (finding Q-R2-4) |

## 9. Limitations

- **No independent observation.** The Generator cannot measure a ramp. Every supplied set is illustrative, and observational verification is OPEN for every run.
- ρ_base and χ₀ are illustrative. ρ, R and 𝟙 are calculated consequences of them, not properties of a ramp.
- The ABR result has no canonical V9 provenance admission (canonical scope). Its integrity rests on IRRS's re-execution, which is integrity, not correspondence.
- IRRS correspondence evaluation (M4) is planned and not operational. Comparison is the student's recorded judgement.
- The `irrs-eval` schema holds one `interior_locus` (P2 is declared there). P3 is declared only in the lesson block.
- Two builds of the Kernel V9 sources are linked: `abr-kernel` (supporting exercise) and `irrs-kernel` (ABR path). `verify-kernel` reports their 10 canonical files equal.

## 10. Questions for Origin

| # | Question |
|---|---|
| Q-R2-1 | **Primary Region and "augmented observation".** The canonical sources define the Primary Region as "the exceptional observational condition in which there is insufficient relational information to instantiate the full A→B→R path" (§10). They do not define "augmented observational circumstances". Document 5 calls the ramp's single relation "the Primary condition", notes that a finer M moves *away* from it, and leaves both the identification and the meaning of augmentation OPEN. Is that the intended treatment, and what does Origin intend by augmented observation (for example ℛ_M = 1 with 𝓜_M = 0)? |
| Q-R2-2 | **Executing with IRRS OPEN.** The program executes A → B → R when `irrs-eval` returns OPEN with 0 violations, citing §5 Step 2 (completeness before execution; correspondence tested afterwards). Should the lesson instead withhold the output until `irrs-eval` returns PassedAwaitingAuthorization, or until it is Authorized? |
| Q-R2-3 | **Illustrative ρ_base = 0.5, χ₀ = 20.0 mm.** These were chosen by the Generator, are taken from no other domain, and have no observational basis. ρ, R and 𝟙 depend on them. Approve, replace, or declare otherwise? |
| Q-R2-4 | **Rings are not refused by canonical code.** A closing relation P4 → P1 passes `irrs-eval` (0 violations), the gate and `DeclaredRelations`, and A → B → R executes, although §8 declares rings inadmissible. The lesson does not add its own refusal, so as not to introduce an admission rule. Should this be addressed in IRRS under a separate authorization, or by a lesson-level refusal Origin declares? |
| Q-R2-5 | **Interior-locus checking.** `irrs-eval` checks only that the declared interior node exists, not that t = target(e) = source(f). The lesson's Step 4 lookup does that against the declared edges. Is the lookup acceptable in the lesson, or should it belong to IRRS? |
| Q-R2-6 | **Other Part I invariants.** ι, ε, τ, N, θ, Φ and J are not computed, because their applicability to this domain is not declared. Should any be taught with this set? |
| Q-R2-7 | **B's observable counterpart.** Document 6 suggests that a student may declare the direct drop P1 → P3 as the comparison target for B[P1 → P2] (as their own correspondence declaration, labelled EXPLANATORY). Is that admissible, or should comparison be limited to A? |
| Q-R2-8 | **Phase 2 posture for a static set.** The supplied sets declare `persistence` with `prior_evidence: open` and no sequence, which leaves 4 Phase 2 conditions OPEN. Is that the intended declaration for a static observation? |

**Carried from revision 1.** Q-1 (A vs Δ) is superseded by Origin's revision direction, since A now executes on a structure where it applies. Q-2 (pathway) is answered provisionally by §5, for confirmation. Q-3 (relation `observable_source`) applies to the supporting exercise only. Q-4 (direction basis): the principal set now uses the marble's observed travel, a process; Q-4 remains for experiments 1–4. Q-5 (VERIFIED by student comparison), Q-6 (IRRS e2e compile failure) and Q-7 (commit records and `Cargo.lock`) are unchanged.

## 11. Generator recommendation

The revised lesson is **ready for independent Verifier review** of its conformance, with Q-R2-2 and Q-R2-4 decided by Origin first. Both determine whether the lesson's execution policy is acceptable. It is **not** observationally verified, and should not be described as verified until a student's measured set and comparison have been recorded and reviewed.

*Bounded over D. No claim beyond D.*
