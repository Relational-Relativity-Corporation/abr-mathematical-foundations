# Lesson 1 — Experiment

> **Revision 2:** this is the experiment for the supporting two-locus exercise. Run its program with `cargo run --bin lesson01 -- <file>`. For the principal exercise see [`04_EXPERIMENTAL_EXERCISES.md`](04_EXPERIMENTAL_EXERCISES.md).

Physical measurement and comparison, step by step. Allow about 25 minutes.

**You need:** a ruler with a millimetre scale, a strip of paper (or a length of tape on a table), a sharp pencil, and somewhere to write your readings. Your notebook entry is the external record your declaration will cite.

---

## Part A — Run the illustrative example first

```
cargo run -- declarations/experiment_1_illustrative.txt
```

Read the report from top to bottom. Find:

- **Step 2:** both readings ADMITTED by the IRRS gate.
- **Step 4:** the canonical function called, and the calculation record.
- **Step 5:** `ADMITTED. Observational status: Synthetic`, and the result Δ[rel.L1_to_L2] = −70.0 mm.
- **Status summary:** observational verification is **OPEN — DEMONSTRATION ONLY.**

The supplied numbers were never measured, so nothing about the physical world has been checked yet. That is what Parts B and C are for.

## Part B — Observe and declare

1. **Make two marks.** Draw two short, thin pencil marks across the paper strip, some centimetres apart. Call the left one **L1** and the right one **L2**.
2. **Read L1.** Lay the ruler along the strip and keep it still. Read the scale value at L1, looking straight down at the mark. Write it in your notebook, for example "p. 4, line 1: L1 = __ mm".
3. **Read L2.** Without moving the ruler, read L2 and write it down on the next line.
4. **Note your conditions.** Note how you read the marks and what limits your reading, such as mark width or the scale graduations. Decide what reading uncertainty you can defend, in mm. This is your declaration. The lesson does not choose a value for you.
5. **Declare.** Copy `declarations/my_measurement_TEMPLATE.txt` to `declarations/my_measurement_1.txt` and fill in every line from your notebook ([`DECLARATION_GUIDE.md`](DECLARATION_GUIDE.md)). Keep `relation = L1 -> L2`, and state in `direction_basis` what you observed about the two marks on the scale that supports that direction.

## Part C — Execute, then compare with an independent observation

1. **Run your declaration:**

   ```
   cargo run -- declarations/my_measurement_1.txt
   ```

   If the summary shows anything OPEN, read the listed conditions, complete your declaration, and run again. Do not change a reading to make a result come out a particular way.

2. **Read the calculated result** in Step 5: Δ[rel.L1_to_L2] = x[L1] − x[L2]. With L2 further along the scale than L1, this is negative. Its size is how far apart the marks are by the scale, and its sign records your declared direction.

3. **Make an independent comparison observation.** Move the ruler so that its zero mark sits exactly on L1. Read the scale at L2 directly. This new reading is the separation, observed without using your two earlier readings. Also note which mark lies further along the scale. Add these lines to your declaration file:

   ```
   comparison_separation  = <direct reading at L2 with zero on L1>
   comparison_uncertainty = <your declared reading uncertainty>
   comparison_order       = L2_further_along_scale
   comparison_method      = ruler zero placed on L1; separation read at L2; notebook p. 4, line 3
   ```

4. **Compare, yourself.** Does the size of the calculated Δ match your directly read separation, within the uncertainty you declared? Does its sign match the order you observed? (For L1 → L2 with L2 further along, a negative Δ matches.) Record your judgement:

   ```
   comparison_outcome = agrees        # or: disagrees
   ```

   The program computes no agreement. The comparison is yours, and you answer for it.

5. **Run again.** The status summary now records your comparison:
   - **VERIFIED:** measured readings, a complete comparison, and your recorded agreement.
   - **DIVERGENT:** you recorded disagreement. That is a legitimate result, not a failure to hide: "divergence is data" (`kernel_reference.md` §16.4). Keep the record, and do not edit the readings to remove the disagreement.

> **Honesty notes.** Both comparison readings use the same ruler and the same marks, so they are not independent in every respect. The IRRS rule is to record overlap, not exclude it ("nothing is independent observably speaking", `abr-irrs` `docs/GOVERNING_DECLARATIONS.md`). Note it in `comparison_method`. And a VERIFIED line here records **your** comparison. IRRS automated correspondence evaluation (stage M4) is planned and not yet operational (`abr-irrs` `README.md`).

## Part D — Second experiment: change one observation

1. **Move one mark.** Erase L2 and draw a new mark further along the strip, or simply make a new mark L2′ and treat it as L2. Leave L1 untouched.
2. **Repeat Part B steps 2–5** for the new reading only, in a new file `declarations/my_measurement_2.txt`, with a new notebook entry. Keep L1's reading and record entry from experiment 1, because that observation has not changed.
3. **Repeat Part C**, including a new direct comparison reading.
4. **Compare the two reports.** Only the observation of L2 changed. The declaration structure, the relation and the canonical function are the same, and the calculated result changed with the observation. Run `declarations/experiment_2_illustrative.txt` to see the same effect with supplied values: −70.0 mm becomes −120.0 mm.

Persistence (how a relation evolves across a sequence of observations), operator R and larger topologies come in later lessons. Two separate runs are two Phase 1 evaluations, not a persistence calculation (`role_separation_and_operator_application.md`).

---

*Bounded over D. No claim beyond D.*
