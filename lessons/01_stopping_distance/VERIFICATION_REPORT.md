# Verification Report — Lesson 1 (Stopping Distance)

**Revision 2, 2026-11-05: Generator return to the Verifier for "Lesson 1 Kernel Conformance Corrections" (V-01 to V-06).** This is the Generator's evidence and self-audit. It is not independent verification. **The lesson is not VERIFIED.**

Classifications, used throughout:
- **DECLARED:** stated explicitly (a source, measurement, topology or parameter). Declared ≠ correct.
- **CALCULATED:** canonical operators, or clearly identified explanatory calculations, executed.
- **SIMULATED:** produced by the declared simulator.
- **VERIFIED:** an outcome independently compared against actual observation within D. **Nothing in this lesson is VERIFIED.**
- **OPEN:** an unresolved declaration, authority decision, admission condition or observational requirement.

Compilation, passing tests, deterministic reproduction, RPPI agreement and V9 admission are conformance, never empirical verification.

## 1. Files changed (from `e0bdd3d`)

| File | Change |
|---|---|
| `src/pipeline.rs` | V-01. An authorization boundary: authorized execution is only performed when the evaluation status is Authorized. Otherwise the report says "AUTHORIZED EXECUTION: NOT PERFORMED" with the canonical rule, and every later output is labelled a NON-AUTHORITATIVE DIAGNOSTIC. The OPEN conditions are kept verbatim (`open_list`) |
| `src/main.rs` | V-01: status table with separate columns (evaluation, authorized?, calculation) and the distinct OPEN conditions. V-02: B outputs relabelled from the canonical B definition, phase accounting, the Exercise A interpretation limit, reconstruction / prediction / comparison labels. V-03: provenance table for every prediction input. V-06: negative fixtures run and recorded. Task 7: status block DECLARED / CALCULATED / SIMULATED / VERIFIED / OPEN. Single-declaration mode prints the authorization status |
| `src/v9.rs` (new) | Kernel V9 provenance admission (`abr_kernel::admission::admit`) of the stopping relation Onset → Standstill, declared alone (Primary, Δ = A formula). Adapted from the removed revision-1 program |
| `src/fields.rs` | V-05: F5 relation bases now declare direction from the simulated position increase, not from the order of state labels. The Onset → Standstill basis is marked Generator-proposed (D-SD-12) |
| `Cargo.toml`, `Cargo.lock` | **Dependency added, stated explicitly:** `abr-kernel` at the pinned `1c1ef70` (the same commit as the IRRS kernel copy). This is needed for V9 `admit`. `Cargo.lock` gains only that entry. No IRRS pin changed |
| `declarations/negative_tests/*.json` (new, 4) | V-06: the negative-path fixtures, previously scratch-only, now preserved in the repository |
| `records/` | Regenerated. New: `expanded_event/V9_stopping_relation.report.txt`, `negative_tests/*.report.txt` (41 files in total) |
| `DEFECT_REPORT_V06_RING_TOPOLOGY.md` (new) | V-06 canonical defect report |
| `TOPOLOGY_REVIEW.md` (new) | V-05: every relation, its simulated basis, the Generator's interpretation, and its authorization status |
| `README.md`, `03_…`, `05_…`, `DESIGN_SPECIFICATION.md` | Wording aligned with the runtime report (V-01, V-02, V-04, Task 7) |

The simulator, the calculations (`sim.rs`, `calc.rs`), all values and the canonical definitions are unchanged.

## 2. Finding-by-finding disposition

| Finding | Disposition |
|---|---|
| **V-01** OPEN evaluation beside successful execution | **Corrected.** An OPEN record cannot be authorized. The authorized path therefore performs nothing for any field. The canonical calls are kept as a NON-AUTHORITATIVE DIAGNOSTIC, labelled so in every report line, field report and status line. The conditions are preserved verbatim in each field report |
| **V-02** observed and predicted distances conflated | **Corrected.** See §3 |
| **V-03** expanded prediction outside the kernel | **Audited.** The runtime report has a provenance table for every input. Three findings are returned OPEN (§4) |
| **V-04** temporal persistence not executed | **Cannot be executed under the pinned authorities. Returned OPEN** (§5) |
| **V-05** relations needing authorization | **Reviewed:** `TOPOLOGY_REVIEW.md`. No relation is authorized. F5 bases are corrected to an observable basis (position increase). **F3 is flagged:** its only simulated basis is time order, which is not a direction declaration (§8). D-SD-2, D-SD-8 and D-SD-12 remain OPEN |
| **V-06** a ring reaches A → B → R | **Preserved and reported.** Fixture `neg4_directed_ring.json`: 15 OPEN conditions, 0 violations; A → B → R executes (now labelled diagnostic). Defect report: `DEFECT_REPORT_V06_RING_TOPOLOGY.md`, with a proposed directed-cycle regression test. No lesson-level rule |

## 3. V-02: stopping-distance interpretation

**Canonical definitions** (`abr-kernel` `1c1ef70`, `src/operators.rs`):
- A[e] = x[source(e)] − x[target(e)].
- B[e] = A[e] + Σ A[f] over f ∈ succ(e), where succ(e) is the set of relations leaving target(e) (`operator_b`).

| Value | Origin | Class |
|---|---|---|
| **174.953 m = −A[Onset → Standstill]** on F5 | **RECONSTRUCTION** from the simulated vehicle states | CALCULATED (diagnostic) from SIMULATED inputs |
| 65.472 m = −B[Onset → ResponseStart] = −(A[Onset → ResponseStart] + A[ResponseStart → BrakeOnset]) | reconstruction: distance from onset to brake onset (perception 53.483 + response 11.990) | as above |
| 109.481 m = −B[BrakeOnset → FullBraking] = −(A[e] + A[FullBraking → Standstill]) | reconstruction: distance from brake onset to standstill (build-up 6.620 + full braking 102.861) | as above |
| 100.187 m | **PREDICTION**, basic Newtonian (Exercise A inputs) | CALCULATED (explanatory) |
| 180.109 m | **PREDICTION**, expanded Newtonian (D-SD-4 OPEN; see V-03c) | CALCULATED (explanatory; inputs are diagnostics) |
| −74.766 m, +5.156 m | **COMPARISON**: prediction minus reconstruction | CALCULATED |

**Correction to revision 1.** The two B outputs were labelled "reaction distance (counterpart of v·t)" and "braking distance (counterpart of v²/2a)". That is not consistent with the phase accounting:
- Exercise A's t = 1.5 s covers perception, response and build-up (the base trial splits it 0.8 + 0.45 + 0.25 s).
- Its v·t therefore spans Onset → FullBraking, which no single B output gives under the declared topology.

The B outputs are now labelled by the states they span.

**State definitions** (`sim.rs`):
- ResponseStart = end of perception;
- BrakeOnset = brake pressure begins;
- FullBraking = full deceleration reached.

Each state position is the first 0.1 ms integration step at or after the declared time.

**Correction to revision 1:** the claim that "A and B do not depend on A-3" was unfounded. On Onset → Standstill, succ and pred are empty, so B and R are A plus empty sums. The A-3 register text names Σ, R and R_persistence. **Whether A-3 covers B's empty successor sum is returned OPEN.**

**Kernel V9 verification of the reconstruction** (`src/v9.rs`, `records/expanded_event/V9_stopping_relation.report.txt`):
- Onset → Standstill was declared alone and run through `execute_primary` (`operator_delta`) and `admission::admit` at `abr-kernel` `1c1ef70`.
- **Result: ADMITTED, observational status Simulated.** Δ = −174.9531225782069 m, bit-identical to F5 A. The reverse traces end at the two Simulated position records. Σ was not evaluated (no ρ_base or χ₀).
- This is provenance conformance, not empirical verification.

## 4. V-03: provenance of the expanded prediction

| Input | Declared observable / parameter | Canonical output | Projection C | Destination |
|---|---|---|---|---|
| initial speed | obs.Body, F1 onset (simulated) | none (node value) | none | reaction and braking phases |
| along-road wind | obs.Air, F1 onset (simulated) | none (node value) | speed × cos(direction), **computed in `sim.rs` before the kernel pipeline** | drag term |
| perception, response and build-up times | obs.Obstacle, Driver, Brake, Tread, F3 | A on the three F3 relations | sign reversal | reaction and build-up phases |
| grade per segment | obs.S00 … S16 elevation, F2a | A[Sk → Sk+1] | atan(−A / spacing) | segment deceleration |
| friction per segment | obs.S00 … S15 friction, F2b | none (node value) | none | segment deceleration |
| station spacing | `simulator_illustrative.json` | — | — | segment length; grade |
| mass, drag area, air density, g | `simulator_illustrative.json` | — | — | drag/m; g·(μcosθ + sinθ) |

**Findings returned OPEN:**
- **V-03a.** The wind's M projection runs in the simulator before submission to the kernel pipeline.
- **V-03c.** The perception, response and build-up times are observed *during* the event, so they are not "conditions available before the stopping event". Whether the expanded calculation qualifies as a prediction is for Origin to decide.

The Newtonian equations are explanatory application mathematics and are not attributed to ABR. **D-SD-4 remains OPEN.**

## 5. V-04: persistence

Kernel V9 has the persistence operators: `operator_a_persistence`, `operator_b_persistence`, `operator_r_persistence`, `operator_e_v5` and `PersistenceState::from_prior`.

**Missing items:**
1. **A wired execution path for persistence.** Nothing currently takes an admitted observation sequence through `operator_e_v5` with RPPI re-execution.
2. **The Origin declaration** that makes a simulated trajectory a Phase 2 sequence: §8, "unless the model is itself declared as a transducer from observable inputs" (D-SD-6).

**Returned OPEN.**

## 6. Canonical execution evidence

| Field | Status | OPEN conditions | Violations | Authorized execution | Diagnostic |
|---|---|---|---|---|---|
| F1 full braking | open | 14 | 0 | not performed | executed; RPPI agreed |
| F1 hazard onset | open | 14 | 0 | not performed | executed |
| F2a elevation | open | 40 | 0 | not performed | executed |
| F2b friction | open | 40 | 0 | not performed | executed |
| F3 response chain | open | 14 | 0 | not performed | executed |
| F5 position | open | 17 | 0 | not performed | executed |
| F5 speed | open | 17 | 0 | not performed | executed |
| F4 × 4 diamonds | open | 15 each | 0 | not performed | executed |

The kinds of OPEN condition:
- the four V9 Phase 2 declarations are absent;
- every node, edge and interior locus is "Proposed — not yet observationally established";
- ρ_base and χ₀ are illustrative values, not admissible for production evaluation.

Full records: `records/*/*.report.txt`. Kernel open conditions surfaced with every R: A-3 (EmptyAdjacency), and the 𝟙 audit HOLD.

## 7. Numerical results before and after

Every operator output in every field report, every trial distance and the reference observation are **identical** to `e0bdd3d`. This was checked by diffing the outputs; the simulator and calculations are unchanged. The only new number is the V9-admitted Δ, which equals F5 A bit for bit.

| | `e0bdd3d` | Now |
|---|---|---|
| Reconstruction −A[Onset → Standstill] | 174.953 m | 174.953 m (labelled diagnostic) |
| Basic prediction / comparison | 100.187 m / −74.766 m | same |
| Expanded prediction / comparison | 180.109 m / +5.156 m | same (V-03c caveat) |
| −B outputs | 65.472 / 109.481 m, labelled reaction / braking | same values, relabelled (V-02) |
| Diamond effects, e.g. tread: dry +2.486 m, wet +33.550 m | as reported | same |

The field-declaration digests for F5 changed, because the basis text changed (V-05).

## 8. Tests and reproduction

| Command (at the pins, clean clones) | Result |
|---|---|
| `abr-kernel` `1c1ef70`: `cargo test` | **112 passed**, 0 failed (61 lib, 3 provenance_demo, 12 v9_admission, 35 v9_conformance, 1 doctest) |
| Lesson: `cargo build --locked` | 0 warnings |
| Lesson: `cargo run --locked`, twice | byte-identical `records/` (41 files) |

The lesson has no test suite of its own. Its checks are the canonical calls and the negative fixtures.

`Cargo.lock` sha256 is `03a7276410af040afc1bc71e4e7e4e507287aa2d843fd689dc1cb8287e48e788`. `records/LESSON1_REPORT.txt` sha256 is `73fe9b9e63305fb059d499a57b3a4fa787554a9901c7fe864265e46a996be759` (LF line endings).

Reproduce:

```
cd lessons/01_stopping_distance
cargo run --locked
cargo run --locked -- declarations/negative_tests/neg4_directed_ring.json
```

## 9. Decisions requiring Origin authorization

| Decision | Item |
|---|---|
| D-SD-2 | relation direction bases (F1; F3 flagged; F2; F5) |
| D-SD-3 | projections C (including the V-03a placement) |
| D-SD-4 | the expanded Newtonian formula |
| D-SD-6 | Phase 2: the simulator as a transducer; plus a wired persistence execution path (V-04) |
| D-SD-7 | all illustrative values |
| D-SD-8 | relations between controlled trials |
| D-SD-10 | the "material" threshold |
| D-SD-12 | the Onset → Standstill relation |
| V-03c | whether the expanded calculation counts as a prediction |
| A-3 | whether it covers B's empty successor sum |
| V-06 | ring topology: directed-cycle check (canonical) |

*Bounded over D. No claim beyond D.*
