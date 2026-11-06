# Preface — An Invitation to Mathematicians and Physicists

**ABR Mathematical Foundations.** First draft, awaiting Origin review.

> Content class: this preface is **[EXPLANATORY]** throughout, except for passages marked **[CANONICAL]** with a source reference. It introduces no definition or rule.

---

## To the reader

This is an invitation, not a manifesto.

You already know how to do mathematics. You know how to state a definition precisely, how to notice when a step does not follow, and how to tell a result that has been proved from a result that has only been believed. In physics, you know the difference between a calculation that is internally consistent and a calculation that agrees with a measurement. None of that changes here. This curriculum asks you to bring those skills with you and apply them to a mathematics that starts in a particular place: with **directed relations declared from observable information**.

We do not ask you to accept anything on authority. The mathematics is implemented, and every operator in it has a reference implementation in Rust with a conformance test suite. Each statement in this curriculum names its canonical source. When a statement is only explanation, it says so. When a question is open, it says that too. Your job, if you accept the invitation, is to check, to question, and to extend the work. That is the job you already do.

> **[CANONICAL]** "The reader remains Origin. This document states constraints. Verification remains available to you directly." — `abr-kernel` `docs/kernel/abr_operators_plain.md`

---

## Human agency

The framework places responsibility in one place: with the person who makes the declarations.

Someone has to decide what is being observed, what counts as a distinction between two loci, which direction a relation runs, and how a measurement becomes a number. Those are scientific judgements. They draw on knowledge of instruments, of physical systems, of what has been measured before and how well. No formula supplies them, and the framework does not pretend otherwise.

> **[CANONICAL]** "Responsibility stays with what has agency — it cannot migrate to operators, documents, frameworks, or models, none of which have any. In the current implementation domain, this is the human." — `role_separation_and_operator_application.md`, Constraint 4
>
> **[CANONICAL]** "The operators hold no agency. Whatever the workflow commits to, a person committed it." — `kernel_reference.md` §1

In the framework's vocabulary, the person who declares is called **Origin**. Origin is a function, not a title: whoever declares the domain, the measurement mapping, and the relations, and takes responsibility for them, is performing it. The canonical sources hold that this function cannot be handed to a machine, a document, or a framework without being given up (`kernel_reference.md` §3).

---

## Scientific creativity

A framework this strict about provenance can look as if it leaves little room for imagination. In practice it moves where imagination is needed.

The operators are fixed. They take a directed difference, accumulate along declared continuation, and couple through observed asymmetry, and they do the same thing every time. What is not fixed, and what takes insight, is the **declaration**:

- which entities in a system are the loci,
- which observed distinctions establish a relation between them,
- what one step of the process is, and what observable property of the process sets its direction,
- what a measurement mapping preserves and what it discards.

A declaration may be wrong. The canonical sources expect this and give it a place: a candidate declaration is executed unchanged, its result is compared with observation, and any divergence is preserved and returned (`kernel_reference.md` §16). Declaring, testing against observation, and revising is a creative and empirical cycle. It is the same cycle that has always driven physics, made explicit and auditable.

> **[CANONICAL]** "Topology is varied in response to comparison with observables, not to obtain a particular numerical result." — `kernel_reference.md` §1

---

## AI as computational augmentation

This curriculum was drafted with an AI system, and the framework was built to be used with them. Their role is defined as precisely as the mathematics.

An AI system can act as a **Generator**, proposing candidate structure within what Origin has declared, and can implement and execute the authorized calculations. It can carry the operators over far more relations, far more observations, and far more variants of a declaration than a person could evaluate by hand. A separate AI system, or a person, can act as a **Verifier**, reading the work from a distinct frame and returning located findings.

What an AI system does not do is decide what was observed, or take responsibility for the declaration.

> **[CANONICAL]** "AI extends the range over which the declared operators may be applied; it does not assume Origin. The operators extend observation beyond unaided human perception where the declared observable permits; they do not transfer agency." — `role_separation_and_operator_application.md`, Origin
>
> **[CANONICAL]** "Scale does not alter declared roles — increasing computational scale, information volume, or processing speed changes how the declared roles are performed. It does not alter what the roles are or who holds them." — `role_separation_and_operator_application.md`, Constraint 8

AI systems bring a specific risk, and the framework names it: mathematics carried in from training data rather than from the declaration. That includes a probability where no ensemble was declared, a "mass" where relational inertia ι was declared, or a default parameter value nobody chose. The canonical sources call this **legacy mathematical intrusion** and make checking for it a standing duty of the Verifier (`kernel_reference.md` §20). The concern is provenance, not the merit of any established mathematics. This curriculum is subject to the same check.

---

## What we ask of you

1. **Check the sources.** Every [CANONICAL] passage gives a file and section in `abr-kernel` (Kernel V9, `1c1ef70`) or `abr-irrs` (`c7410c5`). If our restatement differs from the source, the source governs, and we want to know.
2. **Run the code.** `cargo test` in `abr-kernel` reproduces the conformance baseline. The operators are short enough to read in an afternoon.
3. **Separate what is established from what is open.** The canonical record keeps an explicit register of open conditions (`kernel_reference.md` §18) and a Part II of framework-level statements awaiting derivation (§15). Several of those open conditions are mathematical problems, and closing them by derivation is real work.
4. **Bring your judgement.** If a definition is unclear, a step does not follow, or an explanation here outruns its source, that is a finding.

The rest of the curriculum begins where mathematics has always begun: with observation.

---

*Bounded over D. No claim beyond D.*
