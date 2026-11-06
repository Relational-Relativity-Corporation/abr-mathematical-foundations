# Verification Report — Lesson 1 (Stopping Distance), Generator self-audit

**Prepared by the Generator, 2026-11-05.** This is a self-audit, not independent verification. Verifier review and the Origin decisions D-SD-1 to D-SD-11 are outstanding.

## 1. Origin direction followed

| Origin direction | How it was applied |
|---|---|
| Replace the marble-ramp Lesson 1 with the Newtonian example | `lessons/01_first_relational_calculation/` removed (recoverable from `5ebe041`); `lessons/01_stopping_distance/` added; top-level `README.md` and `CURRICULUM.md` rows updated |
| Data: "I propose, labelled" | Every value is in `declarations/simulator_illustrative.json`, labelled ILLUSTRATIVE and chosen by the Generator, with none copied from another repository |
| "Run ABR using the IRRS as a guide, report your findings" | Every field goes through IRRS evaluation and gate admission. A → B → R runs where IRRS returns OPEN with 0 violations, and would stop on REJECTED. Illustrative ρ_base and χ₀ are declared so that B and R execute, and IRRS flags them OPEN |

## 2. Findings

| # | Finding |
|---|---|
| F-1 | **The canonical operators calculate the stopping distance** on field F5, the vehicle's observed states: −A[Onset → Standstill] = 174.953 m, −B[Onset → ResponseStart] = 65.472 m (reaction), −B[BrakeOnset → FullBraking] = 109.481 m (braking). *Correction:* the first version of this lesson (commit `6d8b9ea`) wrongly stated that no operator gives a stopping distance. It had left the vehicle's own states out of the field. Origin pointed out the error, and F5 was added |
| F-2 | **IRRS returned OPEN with 0 violations for all eleven fields.** Seven event fields: 14, 14, 40, 40, 14, 17 and 17 conditions. Four diamonds: 15 each. The conditions are the absent Phase 2 declarations, "proposed" support (GR-5) and the illustrative parameters. Every reading was admitted by the gate |
| F-3 | **A → B → R executed on every field.** The RPPI re-execution agreed each time. 𝟙[e] = 1.0 on most relations, a review signal under §11.4. On Onset → Standstill, A = B = R and 𝟙 = 0: R there rests on empty sums (A-3) |
| F-4 | **Phase 2 is unreachable.** Changing deceleration needs Phase 2. §8 requires the simulator to be declared a transducer (D-SD-6), and `irrs-session` `c7410c5` has no persistence execution path |
| F-5 | **IRRS raised no objection to relations between separate controlled trials** (the F4 diamonds). Their admissibility is still Origin's decision (D-SD-8): IRRS checks structure, not whether a relation is an observed passage |
| F-6 | **Two of the four diamond interactions are Generator-authored.** Tread × moisture and visibility × attention come from tabled values; wind × speed and grade × surface come from the declared mechanics. Documents 4 and the report state this |
| F-7 | **Results:** basic prediction 100.187 m; expanded prediction 180.109 m (pending D-SD-4); relational 174.953 m (equal to the simulator's own figure, since it is the same observation). The expanded prediction is closer here, but that is agreement with a declared model only |
| F-8 | **Rings are not refused** by `irrs-eval`, `relational-gate` or `irrs-session` at `c7410c5`, although Kernel V9 §8 declares them inadmissible (see §3, negative-path checks). Returned to Origin |

## 3. Self-audit

| Check | Result |
|---|---|
| Kernel operators implemented in the lesson | none: `pipeline.rs` calls only `irrs_eval`, `relational_gate`, `irrs_session` and `relational_isolation` |
| Newtonian formulas attributed to the kernel | none: `calc.rs` and `sim.rs` are labelled [EXPLANATORY] in the code, the documents and the report |
| Interaction formula imposed | none: tables for tread × moisture and visibility × attention, declared mechanics for the other two |
| Improvement manufactured | the comparison is computed, not asserted; the simulator models effects the calculation does not (D-SD-9); the per-trial residuals of both signs are reported |
| Values imported from another domain or repository | none; g is declared as the standard value |
| Canonical repositories modified | none |
| Machine paths or secrets in records | none (checked by grep) |
| Determinism | two consecutive runs gave byte-identical `records/` (36 files) |
| Build | `cargo build --locked`: 0 warnings |

**Negative-path checks.** Each check used a modified copy of `records/expanded_event/F3_response_chain.declaration.json`, run with `cargo run -- <file>`. The copies were kept outside the repository.

| Modification | Result |
|---|---|
| χ₀ removed | IRRS OPEN; readings admitted; **stopped**: "Missing declaration(s): χ₀" |
| Tread reading set to 12 s, outside [0, 10] | gate did not admit it; **stopped** before the kernel |
| Edge to an undeclared node | IRRS **REJECTED** (GR-4); nothing executed |
| Closing relation Tread → Obstacle (a ring) | IRRS OPEN, 0 violations; **A → B → R executed**. The canonical code at these pins does not refuse rings, although §8 declares them inadmissible. This is finding F-8, the same as Q-R2-4 from the removed ramp lesson. The lesson adds no refusal rule of its own |

No real measurement exists.

## 4. Baseline (same pins, run earlier on 2026-11-05)

- abr-kernel `1c1ef70` `cargo test`: 112 passed.
- abr-irrs `c7410c5`: `cargo test --workspace` fails to compile `irrs-e2e-tests` (10 errors; Q-6, unchanged). Excluding it, 189 passed.
- `irrs-eval verify-kernel`: exit 0, 10 files equal.

## 5. Reproduction

```
git clone https://github.com/Relational-Relativity-Corporation/abr-mathematical-foundations.git
cd abr-mathematical-foundations/lessons/01_stopping_distance
cargo run --locked
```

`Cargo.lock` sha256 is `d9a31b9c2eadf4c4b773fdd81e90be0582714a38af82e8a4b1326abf15b5da1b`.

`records/LESSON1_REPORT.txt` sha256 is `6b6281b882ee8fc142d860c4a73295d42f04c7c0246a3348f7c6d61d42ba9a66`. This was produced on Windows, and the stored file uses LF line endings.

## 6. Recommendation

Ready for Verifier conformance review once Origin has decided D-SD-4 (expanded formula), D-SD-6 (Phase 2), D-SD-7 (values), D-SD-8 (relations between trials) and D-SD-12 (the Onset → Standstill relation). It is not observationally verified.

*Bounded over D. No claim beyond D.*
