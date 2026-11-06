# 1 — Introduction to ABR

Content classes: [CANONICAL] is quoted or closely restated from Kernel V9 or IRRS, with a reference. [EXPLANATORY] is teaching text with no mathematical authority. [OPEN] is unresolved. See the [top-level README](../../README.md).

---

## 1.1 The primitive  [CANONICAL]

Kernel V9 rests on one primitive (`kernel_reference.md` §1):

> **A DIRECTED RELATION DECLARED FROM OBSERVABLE INFORMATION.**
>
> Every operator, every invariant, every admissibility condition in this framework exists because of this primitive and cannot be used without it.

Three points about it matter immediately for this lesson.

- **A relation is declared, not computed.** "A relation is declared through M from an observed distinction. Δ and A compute the numerical directed difference over a declared relation; the computation does not declare the relation" (§1, The Chain).
- **Every relation has one admissible direction**, "the direction traceable to an observable through M" (§8).
- **The domain is bounded.** "D contains only what has been declared through M from observable evidence. There is no unbounded extension, no inference beyond the declared boundary, no silent default" (§1).

The mathematics does not tell you which relations exist. You establish them by observation, and you declare them. The mathematics then acts on what you declared, and on nothing else.

## 1.2 The chain  [CANONICAL]

```
Observable → M → numerical projection in D → declared topology
  → operators → calculated result → comparison with observables
```

(`kernel_reference.md` §1.) Each link has a fixed meaning:

| Link | Meaning |
|---|---|
| Observable | What you observe: here, the height of a mark on a ramp |
| M | The declared measurement mapping from the observable into D: here, the ruler reading, unchanged (Identity) |
| D | D := { x ∈ ℝⁿ \| n < ∞, \|x[i]\| < ∞ }. "A value in D is the numerical projection of an observable through M, not the observable itself" |
| Declared topology | The loci and the directed relations between them, each with its observational basis |
| Operators | A, B, R (or Δ, Σ), acting on the declared topology |
| Calculated result | Operator output. It is a calculated result, never itself an observable |
| Comparison | Calculated results set beside independent observations, and the outcome classified (§16.3) |

One more rule runs through the whole lesson: **not evaluated ≠ evaluated with result 0.0** (§1). When something an operator needs has not been declared, the operator does not run, and the program says so. It never fills in a value.

## 1.3 Why start from a complete observable set  [EXPLANATORY]

The operators act on a declared *structure* of relations, not on isolated pairs. A structure with only one relation cannot show what the operators do with relations that follow one another. A structure with several relations in sequence can.

So this lesson starts from a small, complete observable set: four marks on a ramp, connected by the passage of a marble. **Complete** means complete *within the declared domain*. Every mark in D has a reading, and every relation the observation establishes between those marks is declared. It does not mean the ramp is fully known. Its friction, the marble's speed and everything else M discards lie outside D, and the lesson says nothing about them.

From the whole set you will work inward. You will look at what a single relation carries when it is taken alone, and why that brings you to the condition Kernel V9 calls the **Primary Region**: "the exceptional observational condition in which there is insufficient relational information to instantiate the full A→B→R path" (§10). Document 5 covers this, including what this lesson cannot establish about it.

## 1.4 Who does what  [CANONICAL]

Kernel V9 declares three roles (`kernel_reference.md` §3; `role_separation_and_operator_application.md`):

- **Origin** declares D, M, the relations and their provenance, and decides what to do with findings. *In this lesson, you are Origin for your own measured set.*
- **Generator** executes the applicable kernel on what Origin declared. *Here, the lesson program does this by calling canonical code.*
- **Verifier** checks, from a separate frame, that the declarations and execution meet the declared criteria.

The program executes nothing of its own. Every number it prints is either a value you declared or a value returned by a pinned canonical function.

## 1.5 What you will be able to do  [EXPLANATORY]

By the end of Lesson 1 you should be able to:

1. take a small physical arrangement and declare it as a bounded relational structure (D, M, loci, relations, provenance);
2. submit it to IRRS and read what IRRS admits, what it holds OPEN, and why;
3. run the applicable canonical operators and read each output against the declared relations;
4. say what the calculation establishes, and what it does not;
5. compare a calculated result with an independent observation, and record the outcome without overstating it.

Next: [`02_OBSERVABLE_SET.md`](02_OBSERVABLE_SET.md).

*Bounded over D. No claim beyond D.*
