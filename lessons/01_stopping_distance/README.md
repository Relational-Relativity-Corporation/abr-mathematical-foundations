# Lesson 1 — Stopping Distance: a Newtonian Calculation and Its Relational Expansion

> **Status: DRAFT, Verifier revision 2.** All values are illustrative, and the readings are simulated.
> - **IRRS returns OPEN for every declared field, so no IRRS-authorized execution is performed.** Every operator output is a labelled NON-AUTHORITATIVE DIAGNOSTIC.
> - **Nothing is VERIFIED.** Kernel V9 admits the reconstructed stopping distance as provenance-conformant (status Simulated); that is not empirical evidence. It awaits Origin decisions D-SD-1 to D-SD-11 ([`DESIGN_SPECIFICATION.md`](DESIGN_SPECIFICATION.md) §10) and Verifier review.

**Objective.** A simple physical calculation can be expanded into a more complete description of an observable system by bringing in more of the relevant relationships. More relevant knowledge permits a more complete description. Whether that description is accurate is established only by observation.

This is not a contest between mathematical formalisms. The Newtonian calculation is where the lesson starts. The relational field is what the lesson adds.

## Quick start

```
cd lessons/01_stopping_distance
cargo run
```

The program prints the whole lesson report and writes it to `records/LESSON1_REPORT.txt`. It takes under a second after the first build. The first build fetches the pinned canonical crates.

To try your own change, copy a declaration from `records/expanded_event/`, edit it, and run `cargo run -- <your copy>.json`. That runs the one declaration through IRRS and the kernel and prints its report.

## Documents

| | Document | What it covers |
|---|---|---|
| 1 | [The basic calculation](01_BASIC_CALCULATION.md) | Exercise A by hand: d = vt + v²/(2a) = 100.187 m, and its assumptions |
| 2 | [The expanded relational field](02_EXPANDED_RELATIONAL_FIELD.md) | Exercise B: D, M, loci and relations for the twelve additional items |
| 3 | [Canonical operators and the calculation](03_OPERATORS_AND_THE_CALCULATION.md) | What IRRS and A → B → R return, and exactly which outputs reach the stopping distance |
| 4 | [Interdependent conditions](04_INTERDEPENDENT_CONDITIONS.md) | Four controlled-trial diamonds: one condition's significance depending on another |
| 5 | [Observation and verification](05_OBSERVATION_AND_VERIFICATION.md) | Exercise C: the reference event, the side-by-side results, and what remains OPEN |
| — | [Design specification](DESIGN_SPECIFICATION.md) | The design, the decisions returned to Origin, implementation notes |
| — | [Verification report](VERIFICATION_REPORT.md) | The Generator's self-audit, findings and reproduction |

## Results at a glance (illustrative; simulated reference)

| | Stopping distance | Comparison: minus reconstruction | Class |
|---|---|---|---|
| **PREDICTION**, basic Newtonian (Exercise A inputs) | 100.187 m | −74.766 m | CALCULATED (explanatory) |
| **PREDICTION**, expanded Newtonian (D-SD-4 OPEN; uses latencies observed during the event, V-03c) | 180.109 m | +5.156 m | CALCULATED (explanatory; inputs are diagnostics) |
| **RECONSTRUCTION**, −A[Onset → Standstill] on the vehicle's simulated states (event: worn tread, wet road, 3 % downgrade, wind 20° off the nose, dusk, distracted driver) | **174.953 m** | — | CALCULATED, diagnostic (IRRS OPEN); V9 ADMITTED (Simulated) |

The canonical A gives the stopping distance as the directed difference between the vehicle's states. B[e] = A[e] + Σ A over succ(e) gives:
- 65.472 m from onset to brake onset;
- 109.481 m from brake onset to standstill.

These are *not* Exercise A's reaction and braking terms: Exercise A's 1.5 s also covers the brake build-up.

The reconstruction uses the event's own states, so it is not a prediction.

## Files

| Path | Content |
|---|---|
| `declarations/simulator_illustrative.json` | Every simulator and calculation value, each labelled illustrative |
| `src/sim.rs` | Reference simulator [EXPLANATORY, not kernel] |
| `src/calc.rs` | Basic and expanded Newtonian calculations [EXPLANATORY, not kernel] |
| `src/fields.rs` | The declared fields, written in the irrs-eval declaration schema |
| `src/pipeline.rs` | IRRS evaluation → gate admission → `irrs_session::execute` (canonical calls only) |
| `src/main.rs` | Orchestration and the side-by-side report |
| `records/LESSON1_REPORT.txt` | The complete run report |
| `records/expanded_event/` | For each field: the submitted declaration, the report and the IRRS evaluation record, plus the reference trajectory |
| `records/diamonds/` | The four condition-diamond fields, plus a summary for each trial |
| `records/expanded_event/V9_stopping_relation.report.txt` | Kernel V9 provenance admission of the stopping relation |
| `declarations/negative_tests/`, `records/negative_tests/` | Negative-path fixtures and their reports (missing χ₀, out-of-extent reading, undeclared node, directed ring) |
| `src/v9.rs` | Kernel V9 `admission::admit` of Δ[Onset → Standstill] |
| [`TOPOLOGY_REVIEW.md`](TOPOLOGY_REVIEW.md) | Every relation: its simulated basis, the Generator's interpretation, and its authorization status |
| [`DEFECT_REPORT_V06_RING_TOPOLOGY.md`](DEFECT_REPORT_V06_RING_TOPOLOGY.md) | Canonical defect: a directed ring reaches A → B → R |

Canonical authorities: IRRS `abr-irrs` `c7410c5` (`irrs-eval`, `relational-gate`, `irrs-session`, `irrs-kernel` = Kernel V9 `abr-kernel` `1c1ef70`, byte for byte). The pins are fixed by `Cargo.lock`.

*Bounded over D. No claim beyond D.*
