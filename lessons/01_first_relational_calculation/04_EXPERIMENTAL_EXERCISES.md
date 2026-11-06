# 4 — Experimental Exercises

Each exercise changes the observation or the declaration, runs it again, and asks you to read what changed. Content classes as in [Document 1](01_INTRODUCTION.md). Run every command from `lessons/01_first_relational_calculation/`.

---

## Exercise 1 — Run the whole set  (10 min)

```
cargo run -- declarations/set_1_ramp_illustrative.json
```

Find these in the report:

1. the IRRS status and its 14 OPEN conditions (Step 2);
2. the four gate admissions (Step 3);
3. the two interior loci and the kernel they select (Step 4);
4. the output tables (Step 6).

For each OPEN condition, say in one sentence what would have to be *observed* or *declared* to close it.

## Exercise 2 — A second observation set  (15 min)

Set 2 is the same ramp and the same four marks after the support under the top end is lowered (illustrative values).

```
cargo run -- declarations/set_2_ramp_lowered_illustrative.json
```

Actual outputs, set 1 beside set 2 (from `records/`):

| | set 1 | set 2 |
|---|---|---|
| readings P1, P2, P3, P4 (mm) | 240.0, 180.0, 135.0, 110.0 | 210.0, 162.0, 126.0, 110.0 |
| A[P1 → P2], A[P2 → P3], A[P3 → P4] (mm) | 60.0, 45.0, 25.0 | 48.0, 36.0, 16.0 |
| B[P1 → P2], B[P2 → P3], B[P3 → P4] (mm) | 105.0, 70.0, 25.0 | 84.0, 52.0, 16.0 |
| ρ[P1], ρ[P2], ρ[P3], ρ[P4] | 0.375, 0.375, 0.34615384615384615, 0.2777777777777778 | 0.35294117647058826, 0.35294117647058826, 0.32142857142857145, 0.2222222222222222 |
| R[P1 → P2], R[P2 → P3], R[P3 → P4] (mm) | 131.25, 40.0, 0.76923076923077 | 102.3529411764706, 28.0, −0.7142857142857153 |
| 𝟙 on each relation | 1.0, 1.0, 1.0 | 1.0, 1.0, 1.0 |
| IRRS evaluation | OPEN, 14 conditions, 0 violations | OPEN, 14 conditions, 0 violations |

What to notice [EXPLANATORY]:

- The topology is unchanged. The same marble passages hold, so the same relations are declared and the same kernel applies. Only the readings changed.
- Every A is smaller in set 2. Each relation's directed difference follows the readings at its two ends.
- R on P3 → P4 changes sign, from +0.769… to −0.714…. That relation has no declared successor, so its R depends on the empty-sum condition A-3 (OPEN). Its value also depends on the illustrative ρ_base and χ₀. Record the change, and do not interpret it as a property of the ramp.

**These are two separate declarations, not a sequence.** No relational process step connects set 1 to set 2, so the persistence operators (A_persistence and the rest) do not apply (§8). The table above is your comparison of two calculations. It is not a kernel operation. Any difference you state comes from reading the two reports.

## Exercise 3 — Leave out a required declaration  (5 min)

Set 3 is set 1 with ρ_base and χ₀ not declared.

```
cargo run -- declarations/set_3_parameters_undeclared_illustrative.json
```

The run stops at Step 5:

```
  STOP at the missing declaration(s), returned to Origin. Nothing is substituted:
    not declared: ρ_base (abr_topology.operator_parameters.rho_base)
    not declared: χ₀ (abr_topology.operator_parameters.chi_0)
```

Explain why the program does not run A and B and leave R out. Use `kernel_reference.md` §9 ("Where these values are not declared by M, the operator is not evaluated and there is no operator result") and §10 ("R is not optional").

## Exercise 4 — One relation alone  (10 min)

Take only P2 and P3 from set 1, with the one relation between them. This is the supporting two-locus program from revision 1:

```
cargo run --bin lesson01 -- declarations/experiment_5_ramp_segment_illustrative.txt
```

Actual result: V9 admission ADMITTED (observational status `Synthetic`), **Δ[P2 → P3] = 45.0 mm**, and Σ *not evaluated* (ρ_base, χ₀ not declared). No interior locus is established, so A → B → R does not apply. The Primary kernel's Δ applies.

Compare the value with A[P2 → P3] in Exercise 1. They are equal, because "Δ is A under a declared renaming — formula identical" (`operators.rs`). What the single relation does not carry is any continuation: no B and no R. Document 5 takes this further.

The four revision-1 declarations (`experiment_1` to `experiment_4`) still run with the same command. They show a second pair, a refused A → B → R request, and an incomplete declaration ([`SUPPORTING_PRIMARY_PAIR.md`](SUPPORTING_PRIMARY_PAIR.md)).

## Exercise 5 — Your own ramp  (30 min)

1. Prop a grooved ramp on books. Draw four marks beside the groove, unevenly spaced, from the high end down. Label them P1 to P4.
2. Release a marble above P1 several times. Write down what you see: does it pass P1, P2, P3, P4 in that order, along the groove, every time? Your notebook page is your evidence. Give it an identifier, for example `notebook-2026-11-05-p1`.
3. Measure the height of each mark above the tabletop, with the ruler standing vertically on the table. Read each one twice, and write down the readings and how certain you are of them.
4. Copy `declarations/my_set_TEMPLATE.json` to `declarations/my_set_<name>.json` (this name is git-ignored), then fill it in.
   - Replace every `<…>` and every `null`.
   - Set `observational_support` to `observed` only for a mark or passage you actually observed, and add an `evidence` entry naming the notebook page for each.
   - Declare ρ_base and χ₀ yourself, with provenance `declared`, and say in `source_identity` why you chose them. Do not copy the lesson's values or values from any other domain.
5. Run it with `cargo run -- declarations/my_set_<name>.json`.
6. Make the comparisons in [Document 6](06_VERIFICATION_GUIDE.md), record them in the `comparisons` block, and run again.

Then move one mark (or change the ramp's support), measure again, and repeat as a second set. Compare the two reports as in Exercise 2.

## Exercise 6 — Declaration discipline  (on paper, 5 min)

Answer from `kernel_reference.md` §8, without running anything:

1. Why is P1 → P3 not declared, although the marble goes from P1 to P3?
2. What independent observation would be needed to declare P2 → P1?
3. Why is P4 → P1 inadmissible?

[OPEN] At the pinned commits, no canonical code refuses a closing relation such as P4 → P1. IRRS evaluation, the gate and the kernel's `DeclaredRelations` all accept one, and A → B → R executes over it. The rule in §8 is enforced by you, as Origin, when you declare. This is returned to Origin as finding Q-R2-4 (VERIFICATION_REPORT.md). Do not use such a run as a result.

You can also try a reading outside the ruler's scale (for example P1 = 350). The gate reports "value 350 outside declared extent [0, 300] — recorded, not admitted", and nothing reaches the kernel.

Next: [`05_TOWARD_THE_PRIMARY_REGION.md`](05_TOWARD_THE_PRIMARY_REGION.md).

*Bounded over D. No claim beyond D.*
