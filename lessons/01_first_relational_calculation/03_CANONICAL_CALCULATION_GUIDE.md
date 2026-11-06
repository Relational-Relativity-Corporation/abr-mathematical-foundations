# 3 — Canonical Calculation Guide

Stage B (submit), Stage C (execute) and Stage D (examine). Content classes as in [Document 1](01_INTRODUCTION.md).

---

## 3.1 Run it

```
cargo run -- declarations/set_1_ramp_illustrative.json
```

The report is printed and saved as `records/set_1_ramp_illustrative.report.txt`. The complete IRRS evaluation record is saved as `records/set_1_ramp_illustrative.irrs_eval.json`. The report has eight steps and a status summary. Each step names the canonical function it calls.

## 3.2 The eight steps  [EXPLANATORY description of CANONICAL calls]

| Step | What happens | Canonical code (`abr-irrs` `c7410c5` unless stated) | What stops the run |
|---|---|---|---|
| 1 Declaration | Your file is read and echoed: D, M, loci, relations, interior loci, parameters | — | invalid JSON or schema; no `lesson` or `abr_topology` block |
| 2 IRRS declaration evaluation | IRRS evaluates the whole declaration and returns a record: status, every OPEN condition, every violation | `irrs_eval::Evaluator::evaluate` (`irrs-eval/src/evaluator.rs`) | **REJECTED** (an established contradiction, such as a relation to an undeclared locus) |
| 3 IRRS observation admission | The region is established from your recorded `observed_locus`. Each reading is submitted to the gate, which executes M itself and checks the extent | `relational_gate::RegionBuilder::establish`, `AdmissionGate::submit` | any reading not ADMITTED |
| 4 Kernel selection | Each interior locus you declared is looked up: its incoming relation must be declared and end there, its continuing relation must be declared and start there | rule: `kernel_reference.md` §10 | interior loci established but a non-ABR selection declared (REJECTED); no interior locus established (Primary condition: see Document 5) |
| 5 Completeness | Every input A → B → R uses must be declared: ρ_base, χ₀, a provenance class for every relation, the 𝟙 tolerance | rule: `kernel_reference.md` §5 Step 2, §9 | any of these missing: **stop at the exact missing declaration** |
| 6 Execution | IRRS calls the canonical operators, then re-executes them independently and compares every output | `irrs_session::execute::execute` → `operator_a`, `compute_rho`, `operator_b`, `operator_r`, `detection_indicator` (`irrs-kernel`, byte for byte with `abr-kernel` `1c1ef70`) | IRRS structural error; RPPI divergence |
| 7 V9 provenance admission | Reported, not called: canonical V9 admission covers the Primary kernel only | `abr-kernel` `src/admission.rs`: "OPEN: ABR kernel A → B → R" | — |
| 8 Comparison | Your recorded comparisons are printed beside the calculated values | none: the program computes no agreement | — |

**IRRS OPEN does not stop execution, and REJECTED does.** In set 1, IRRS returns OPEN with 14 conditions and 0 violations. Each OPEN condition says that something is not *observationally established* (illustrative support, illustrative parameters, Phase 2 undeclared), not that something is missing for the calculation. [CANONICAL] Kernel V9 separates the two: "Declaration completeness is checked before execution. Declaration correspondence is tested through execution and comparison with observables … If all required declarations exist: execute the applicable kernel (section 10) unchanged" (§5 Step 2). The calculation therefore runs, and every result carries the IRRS OPEN status with it. [OPEN] Whether a lesson should instead withhold A → B → R output until IRRS returns PassedAwaitingAuthorization is returned to Origin (VERIFICATION_REPORT.md, Q-R2-2).

## 3.3 The operators that apply  [CANONICAL]

Two interior loci are established, so the ABR kernel applies, and "R is not optional" (§10). The forms are from `kernel_reference.md` §11.1:

| Operator | Form | Here |
|---|---|---|
| A | A(x)[e] = x[s] − x[t] for each declared edge e = (s, t) | directed difference of heights along each declared relation, in its declared direction |
| ρ | ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i]); χ[i] = max \|A(x)[e]\| over edges incident to i | one value per locus, from the illustrative ρ_base and χ₀ |
| B | B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]; immediate successors only; terminal edges accumulate nothing | each relation together with its declared immediate continuation |
| R | R(g)[e] = g[e] + ρ[src(e)]·(Σ_{succ(e)} g − Σ_{pred(e)} g) | each relation's B, plus the source locus's ρ times (B of what follows − B of what precedes) |
| 𝟙[e] | 𝟙[e] = 1 iff the antisymmetric term of R at e ≠ 0 (§13.4); IRRS reads it as `detection_indicator(R[e] − B[e], tolerance)` | 1.0 or 0.0 per relation |

Not applicable here: Δ and Σ (the Primary kernel: Document 5), and A_persistence, B_persistence, R_persistence (Phase 2: no declared sequence of observations, §8).

## 3.4 Set 1 outputs (actual, illustrative inputs)  [CALCULATED]

From `records/set_1_ramp_illustrative.report.txt`:

| Relation | A (mm) | B (mm) | R (mm) | 𝟙 |
|---|---|---|---|---|
| P1 → P2 | 60.0 | 105.0 | 131.25 | 1.0 |
| P2 → P3 | 45.0 | 70.0 | 40.0 | 1.0 |
| P3 → P4 | 25.0 | 25.0 | 0.76923076923077 | 1.0 |

| Locus | ρ (dimensionless) | declared incoming | declared outgoing |
|---|---|---|---|
| P1 | 0.375 | — | P1 → P2 |
| P2 | 0.375 | P1 → P2 | P2 → P3 |
| P3 | 0.34615384615384615 | P2 → P3 | P3 → P4 |
| P4 | 0.2777777777777778 | P3 → P4 | — |

Structural check `relational_isolation(rel)` = false. IRRS's independent re-execution agreed on every R and 𝟙 output.

## 3.5 Stage D — what the results establish  [EXPLANATORY]

Keep three kinds of number apart:

| Kind | In set 1 | Status |
|---|---|---|
| **Declared values** (measured, or here illustrative) | the four heights; ρ_base, χ₀, tolerance | DECLARED |
| **Calculated relationships** | A, B, ρ, R | CALCULATED: kernel outputs, not observables |
| **Derived invariant** | 𝟙[e] | Part I invariant, support *Derived* (§13.4); read from the R output |

**A, the directed difference on each relation.** Each A value belongs to one declared relation and follows that relation's declared direction. All three are positive in set 1: along each observed passage of the marble, the reading at the source is greater than the reading at the target. A is the calculated counterpart of something you can observe directly: the drop in height from one mark to the next. That is why it is the first quantity to compare (Document 6).

**B, each relation with its continuation.** B[P1 → P2] combines A on P1 → P2 with A on its one declared successor, P2 → P3. P3 → P4 has no declared successor, so B equals A there: "terminal edges accumulate nothing". B uses the interior loci. Without P2 and P3 as interior loci, there would be nothing to accumulate.

**ρ, one value per locus.** Each ρ comes from the largest |A| on a relation touching that locus, through the saturating form with the illustrative ρ_base and χ₀. Two parts of this are OPEN in the kernel itself: the max-selection rule and the saturating form (§11.2). IRRS prints both with every run.

**R, local balance.** R adds to each relation's B the source locus's ρ times the difference between the B of what follows and the B of what precedes it. On P2 → P3 the preceding B (105.0) is larger than the following B (25.0), so R is lower than B. On P1 → P2 nothing precedes, and on P3 → P4 nothing follows. Those two R values therefore use a sum over an empty declared set, which evaluates to 0.0. That is open condition **A-3** (empty adjacency, OPEN, §18), and IRRS reports it.

**𝟙 = 1.0 on every relation.** The antisymmetric term of R is nonzero on all three. [CANONICAL] "R_anti ≠ 0 → REVIEW SIGNAL. Not automatically a failure, inadmissibility, discovery, or confirmation of a physical phenomenon" (§11.4). The meaning of 𝟙 = 0 is itself on HOLD pending A-3 (§13.4).

**What the calculation does not establish.** It does not establish that the ramp has any property named ρ or R. Those values follow from parameters chosen for illustration. It does not establish anything about the marble's motion: speed and time are discarded by M. And it does not establish anything outside D.

## 3.6 Other canonical invariants  [OPEN]

Kernel V9 Part I also defines ι[v], ε[e], τ[v], N[v], θ[e], Φ[v], Φ[S] and J[v] (§13). Each needs declared quantities that this domain does not declare (for example relational inertia, process intervals or bond structure), or a declared correspondence for this domain. Their applicability to a ramp of height readings is not declared in the canonical sources. They are not computed here, and the question is returned to Origin (Q-R2-6). Relational distinguishability (I-S, §15.3) is read from a Δ output, and its implementation `are_proportional` is on HOLD. It is not evaluated.

Next: [`04_EXPERIMENTAL_EXERCISES.md`](04_EXPERIMENTAL_EXERCISES.md).

*Bounded over D. No claim beyond D.*
