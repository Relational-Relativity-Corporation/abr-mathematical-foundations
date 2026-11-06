# Lesson 1 — Expected Results

> **Revision 2:** these are the revision-1 outputs of the supporting two-locus exercise, preserved unchanged. They regenerate byte for byte with `cargo run --bin lesson01 -- <file>`. The principal exercise's outputs are in [`VERIFICATION_REPORT.md`](VERIFICATION_REPORT.md) §4 and `records/set_*`.

These are the outputs the lesson executable actually produced. The Generator ran it on 2026-11-05 on Windows 11 with `cargo 1.97.1` and `rustc 1.97.1`. The full reports are preserved in `records/`. **All inputs here are illustrative.** None is an observation, and none of these results is experimentally verified.

## Pinned revisions (as resolved in `Cargo.lock`)

```
abr-kernel:      git+https://github.com/Relational-Relativity-Corporation/abr-kernel?rev=1c1ef70a69b3175a2df0458d0ebf1f71475a3d62#1c1ef70a69b3175a2df0458d0ebf1f71475a3d62
relational-gate: git+https://github.com/Relational-Relativity-Corporation/abr-irrs?rev=c7410c590d04a96612619e639afdce7f0751aeca#c7410c590d04a96612619e639afdce7f0751aeca
```

## Summary

| Run | Inputs (illustrative) | IRRS admission | V9 admission | Canonical result | Observational verification |
|---|---|---|---|---|---|
| 1 `experiment_1_illustrative` | L1 = 20 mm, L2 = 90 mm, L1 → L2, Primary | ADMITTED (both) | ADMITTED, status `Synthetic` | **CALCULATED** Δ[rel.L1_to_L2] = **−70.0 mm** | OPEN (demonstration only) |
| 2 `experiment_2_illustrative` | L2 changed to 140 mm | ADMITTED (both) | ADMITTED, status `Synthetic` | **CALCULATED** Δ[rel.L1_to_L2] = **−120.0 mm** | OPEN (demonstration only) |
| 3 `experiment_3_abr_request_illustrative` | As run 1, `kernel_selection = abr` | ADMITTED (both) | not reached | OPEN. `execute_primary` returned `NotExecutable { reason: "kernel selection is not Primary" }`; operator A not evaluated | OPEN |
| 4 `experiment_4_incomplete_illustrative` | As run 1, `measurement_conditions` empty | ADMITTED (both) | **OPEN**: `MissingMeasurementConditions` for `obs.L1` and `obs.L2` | OPEN; value withheld | OPEN |

In runs 1 and 2, Σ was **not evaluated** (`NotEvaluated { step: Sigma, missing: ["rho_base", "chi_0"] }`). That is not a result of 0.0.

**Sign convention.** Δ(x)[e] = x[s] − x[t] for the declared e = (s, t). With L1 → L2 the canonical result is x[L1] − x[L2] = 20 − 90, which is −70.0 mm. In a Generator check outside the lesson set, the relation was declared L2 → L1 with the locus order unchanged, and the canonical result was +70.0 mm. The sign follows the declared relation, not the order of the loci.

## Run 1 — full report

`cargo run -- declarations/experiment_1_illustrative.txt` (exit code 0):

```
ABR Mathematical Foundations — Lesson 1: First Relational Calculation
Declaration file: declarations/experiment_1_illustrative.txt

== Canonical authorities (resolved by Cargo.lock) ==
  abr-kernel: git+https://github.com/Relational-Relativity-Corporation/abr-kernel?rev=1c1ef70a69b3175a2df0458d0ebf1f71475a3d62#1c1ef70a69b3175a2df0458d0ebf1f71475a3d62
  relational-gate: git+https://github.com/Relational-Relativity-Corporation/abr-irrs?rev=c7410c590d04a96612619e639afdce7f0751aeca#c7410c590d04a96612619e639afdce7f0751aeca

== Step 1 — Declaration (Origin: the student) ==
  (the 20 declared keys are echoed here; see records/experiment_1_illustrative.report.txt)
  Observation status: ILLUSTRATIVE — supplied demonstration values, NOT an observation (V9 OriginKind::SyntheticFixture)

== Step 2 — IRRS observation admission ==
  Called: relational_gate::RegionBuilder::establish, relational_gate::AdmissionGate::submit
          (abr-irrs c7410c5: relational-gate/src/region.rs, relational-gate/src/gate.rs)
  Region 'lesson1.ruler_scale' established; extent [0.0, 300.0] mm; evidence reading 20.0 mm
  L1: ADMITTED — source obs.L1, mapping m.lesson1.identity, raw input 20.0, gate-computed 20.0 mm
  L2: ADMITTED — source obs.L2, mapping m.lesson1.identity, raw input 90.0, gate-computed 90.0 mm

== Step 3 — Canonical declaration records (abr_kernel::provenance) ==
  Loci (node index = position; labels only, NOT topology): [locus.L1, locus.L2]
  Declared relation rel.L1_to_L2: locus.L1 → locus.L2 (class Continuation), M m.lesson1.identity (Identity), domain domain.lesson1.ruler_scale
  Kernel selection: Primary; ρ_base, χ₀: not declared

== Step 4 — Canonical execution ==
  Called: abr_kernel::provenanced_exec::execute_primary(&registry, "st.lesson1")
          (abr-kernel 1c1ef70, src/provenanced_exec.rs). For a Primary selection it
          calls abr_kernel::operators::operator_delta (src/operators.rs): Δ(x)[e] = x[s] − x[t]
  Calculation record (Generator's account, to be checked by admission):
    input  locus.L1 ← obs.L1 via m.lesson1.identity: observed 20.0, projected 20.0
    input  locus.L2 ← obs.L2 via m.lesson1.identity: observed 90.0, projected 90.0
    relation rel.L1_to_L2: node 0 → node 1, class Continuation
    path   [Delta]

== Step 5 — V9 provenance admission ==
  Called: abr_kernel::admission::admit(&submitted, &registry) (abr-kernel 1c1ef70, src/admission.rs)
  ADMITTED. Observational status: Synthetic
  Comparison rule: exact f64 bit equality: same declared calculation, same frozen function, same execution order, bit-identical declared inputs
  Entry Delta[rel.L1_to_L2] component 0 = -70.0 mm
    contributing declarations: locus.L1, locus.L2, rel.L1_to_L2
  Reverse trace Delta[rel.L1_to_L2] terminates in:
    obs.L1 (SyntheticFixture) — ILLUSTRATIVE EXAMPLE — Lesson 1 supplied values, not an observation / Lesson 1 illustrative value L1
    obs.L2 (SyntheticFixture) — ILLUSTRATIVE EXAMPLE — Lesson 1 supplied values, not an observation / Lesson 1 illustrative value L2
  Not evaluated (not 0.0): NotEvaluated { step: Sigma, missing: ["rho_base", "chi_0"] }

== Step 6 — Comparison with observation ==
  Calculated (admitted): Δ[rel.L1_to_L2] = x[L1] − x[L2] = -70.0 mm
  comparison_separation    = <not declared>
  comparison_uncertainty   = <not declared>
  comparison_order         = <not declared>
  comparison_method        = <not declared>
  comparison_outcome       = not_compared
  The program computes no agreement. The comparison outcome is the student's own
  recorded judgement (EXPERIMENT.md), made against an independent observation.

== Status summary ==
  Code execution            : completed (this report was produced)
  Declaration               : DECLARED (every Lesson 1 key present; admission decides admissibility)
  IRRS observation admission: ADMITTED — both readings (AdmittedObservation<f64>, identity M, inside declared extent)
  V9 provenance admission   : ADMITTED — observational status Synthetic (a provenance-conformance statement, not a correspondence claim)
  Canonical calculation     : CALCULATED — Δ[rel.L1_to_L2] = -70.0 mm (operator_delta, admitted)
  Operator A (ABR kernel)   : not evaluated under this declaration: kernel selection is Primary (no interior locus). The canonical operator applied is Δ, 'A under a declared renaming', formula identical (operators_notation_and_constraint.md); B, R and Σ are not evaluated
  Observational verification: OPEN — DEMONSTRATION ONLY. The readings are illustrative, so agreement with them is not an observation and cannot be VERIFIED
```

## Your own measurement

Your result will differ, because it is your observation. Check that your report shows ADMITTED at both admission stages and CALCULATED for Δ before you compare (EXPERIMENT.md, Part C). For L1 → L2 with L2 further along the scale, expect a negative Δ.

*Bounded over D. No claim beyond D.*
