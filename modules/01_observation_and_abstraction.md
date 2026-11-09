# Module 1 — Observation and Abstraction

**ABR Mathematical Foundations.** First draft, awaiting Origin review.

Mathematical authority: Kernel V9 (`abr-kernel` `1c1ef70`). Content classes are defined in [`README.md`](../README.md). Each section carries its class in its heading.

---

## Aims of this module

By the end of this module the reader should be able to:

1. Say why abstraction was historically necessary for mathematics, and what it achieved.
2. State the observational starting point of Kernel V9: change as the observable constant, and the directed relation declared from observable information as the primitive.
3. State the central distinction of this curriculum: **relation is the mathematical primitive; the primitive does not, by itself, identify or establish any particular observable relationship.**
4. Say who is responsible for observational declarations, and what AI systems do in the workflow.

This module states no new definitions. The formal definitions begin in Module 2 and are taken from the canonical sources.

---

## 1.1 Where mathematics begins — [EXPLANATORY]

Mathematics did not begin with axioms. It began with people noticing things: that one heap holds more than another, that a shadow grows as the sun goes down, that one star rises after another in a fixed order, that a field marked out with a cord has corners that can be made the same every time.

Each of these is an observation of a **difference**: more and less, earlier and later, longer and shorter, here and there. Before there were numbers there were comparisons. Counting is a way of recording, and then reasoning about, a comparison between a collection and a standard sequence of marks or words.

This module holds two things together:

- Mathematics grew out of observation.
- Mathematics grew great by abstracting away from observation.

Both hold. The rest of this curriculum depends on keeping both in view.

---

## 1.2 The achievement of abstraction — [EXPLANATORY]

Abstraction is the act of keeping the structure of a situation and setting aside its particular instance. The history of mathematics is, to a large extent, the history of that act, and of what it made possible. Abstraction was not a mistake or a detour. It was necessary, and its achievements are among the greatest of human thought.

A brief and incomplete record:

- **Place-value numeration.** Babylonian scribes of the early second millennium BCE wrote numbers in a sexagesimal place-value system. Tablets such as Plimpton 322 (c. 1800 BCE) record systematic numerical relationships among the sides of right triangles. A number written this way no longer belongs to any particular heap of grain. It can be compared, multiplied, and tabulated in its own right.
- **Proof from stated assumptions.** Euclid's *Elements* (c. 300 BCE) organised geometry as definitions, postulates, and common notions, followed by propositions proved from them. Its achievement was to make every step answerable to what had been stated before. That ideal of traceability is one this curriculum shares.
- **Algebra.** Al-Khwārizmī's treatise on calculation by completion and balancing (c. 820 CE), whose title gave us the word *algebra*, treated whole classes of problems by a single procedure, independent of the particular quantities involved.
- **Coordinates.** Descartes's *La Géométrie* (1637) joined geometry and algebra, so that a curve could be studied as an equation and an equation pictured as a curve.
- **The calculus.** Newton and Leibniz, independently in the later seventeenth century, gave methods for reasoning about continuous change. Those methods carried physics from Kepler's laws to the mechanics of the solar system and beyond.
- **Freedom from a single geometry.** Lobachevsky (1829) and Bolyai (1832) published consistent geometries in which Euclid's parallel postulate fails. Riemann's 1854 lecture on the hypotheses underlying geometry made space itself a structure to be chosen and examined. Six decades later, Einstein's general theory of relativity (1915) used Riemannian geometry to describe gravitation.
- **Foundations.** Cantor's theory of sets (1870s–1890s) and Hilbert's *Grundlagen der Geometrie* (1899) showed that a mathematical theory could be developed from axioms whose terms need not refer to anything in particular. Hilbert's point was that "points", "lines", and "planes" could be replaced by any objects satisfying the axioms.
- **Structure and physics.** Maxwell's equations (1860s) unified electricity, magnetism, and light in one mathematical structure. Noether's theorem (1918) connected the symmetries of a physical theory to its conserved quantities.

In 1960 Eugene Wigner wrote of the "unreasonable effectiveness of mathematics in the natural sciences." That effectiveness is real. It comes from abstraction: from the willingness to study structure apart from any one instance of it.

---

## 1.3 What abstraction sets aside — [EXPLANATORY]

Every abstraction keeps something and discards something. That is not a defect. It is what makes abstraction work. But it means that when abstract mathematics is brought back to an observed system, the step back is a separate act, and it carries responsibilities of its own.

Two features of the abstract tradition matter for what follows.

**First, objects come before relations.** In the usual set-theoretic foundation, one begins with elements and sets. A relation between sets A and B is then *defined* as a subset of the Cartesian product A × B: a collection of ordered pairs of objects that already exist. The relation is built from the objects. The canonical sources state the contrast directly:

> **[CANONICAL]** "Standard mathematics constructs relations from objects. This framework takes the directed relation itself as primitive and derives everything else from it." — `kernel_reference.md` §1

**Second, the step from structure to world is not part of the structure.** Hilbert's axioms do not say which physical things are points. A differential equation does not say which measured quantity its variable is. Making that link has always been the work of the scientist, done in the laboratory and the notebook rather than in the theory. The abstract tradition, by design, leaves it outside the mathematics.

Kernel V9 starts from that second point. It does not reject the abstract tradition. It makes the link between observation and number an explicit, declared, and checkable part of the work.

---

## 1.4 The observational starting point — [CANONICAL]

Kernel V9 begins with human observation within a bounded domain.

> **[CANONICAL]** "Change is the observable constant. Relation is primitive within the bounded declared domain." — `kernel_reference.md` §2 (Origin declaration, 2026-09-29)
>
> **[CANONICAL]** "Change is the primary observable. Before any variable is declared. Before any locus is named. Before any operator acts. What M reports — at every scale, in every domain, across every declared region — is that something differs from something else." — `abr_operators_plain.md`

The canonical record gives three characteristics, drawn from human experience, which together determine the choice of primitive (`kernel_reference.md` §1):

1. **Complexity.** Observable systems are not reducible to a single governing variable or a universal parameter set, so domain-specific declaration is structurally unavoidable.
2. **Change as the only observable constant.** What is consistently observable across domains is not a stable object but change. Nothing in the framework assumes a stable substrate beneath the change.
3. **Bounded domain of analysis.** Every declaration is bounded. D contains only what has been declared through M from observable evidence.

From these the canonical record concludes:

> **[CANONICAL]** "These three characteristics — complexity, change, and bounded domain — jointly determine that the correct primitive is a DIRECTED RELATION, not a point, not a set, not an object with properties." — `kernel_reference.md` §1
>
> **[CANONICAL]** "The foundational observable primitive is a directed relation declared from observable information. A numerical value is not itself a primitive; it is the projection of an observed relational distinction through a declared measurement mapping M. Observed relations, measurement mappings M, numerical projections in D, and calculated results are held distinct." — `operators_notation_and_constraint.md`, Foundational primitive

The governing order (`kernel_reference.md` §2) is:

```
human observation → bounded domain → observable distinction / change
  → declared relation → M → numerical projection in D → declared topology
  → applicable operators → calculated result → comparison with observables
```

Module 2 gives each term in this chain its formal definition.

---

## 1.5 The central distinction — [CANONICAL] with [EXPLANATORY] commentary

This is the most important idea in the module:

> **Relation is the mathematical primitive. The primitive does not, by itself, identify or establish any particular observable relationship.**

The two halves of this sentence do different work.

### The first half: relation is the primitive — [CANONICAL]

The directed relation is where the mathematics starts. It is not derived from points, sets, or objects with properties. The operators are defined on it and depend on it:

> **[CANONICAL]** "The operators (A, B, R, Δ, Σ) are not independent mathematical objects. They are the forms that act on declared directed relations. … The operators exist because of the primitive. The primitive does not exist because of the operators." — `kernel_reference.md` §1

### The second half: the primitive does not establish particular relations — [CANONICAL]

Having a primitive called "directed relation" does not tell you that any particular relation holds in a system. It does not say that this locus is related to that one, in this direction, at this step of the process. Which relations are declared in a given system is established by observation, through a declared measurement mapping, by Origin. The mathematics never supplies it.

The canonical sources state this in several places, each closing a different route by which the mathematics might seem to supply a relation:

| What the mathematics does not supply | Canonical statement |
|---|---|
| The relation itself | "A relation is declared through M from an observed distinction. Δ and A compute the numerical directed difference over a declared relation; the computation does not declare the relation." — `kernel_reference.md` §1 |
| Existence claims | "The kernel makes no declaration about what exists or does not exist." — `kernel_reference.md` §1 |
| The direction of a relation | "Every declared relation has exactly one admissible direction — the direction traceable to an observable through M. … The reverse direction requires independent observable provenance." — `operators_notation_and_constraint.md`, Relational direction |
| Continuation, boundaries, results | "No continuation, predecessor, successor, boundary closure, or operator result may be supplied by the mathematics itself. Every element of the admissible interior must be traceable to an observable through M." — `operators_notation_and_constraint.md`, Full-operator admissibility condition |
| Missing information | "Where information required by an operator has not been declared through M for the calculation, that operator is not evaluated. No substitute value or classification is introduced." — `kernel_reference.md` §1 |
| Parameter values | "The kernel states forms only. It holds no numerical value for any operator parameter and supplies no defaults." — `kernel_reference.md` §9 |
| A prior state for persistence | "No prior value is supplied where no prior observation was declared." — `operators_notation_and_constraint.md`, Domain |
| Revision of the declaration | "The kernel does not vary the declaration." — `kernel_reference.md` §1 |
| A traceable relation, from computation alone | "A relation that cannot be traced to an observable through M is fabricated within D, even when it computes." — `operators_notation_and_constraint.md`, Relations |

### Commentary — [EXPLANATORY]

A comparison may help, as long as it is taken only as a comparison. A grammar fixes what form a sentence must take, but it does not tell you which sentences are accurate reports of what happened. Knowing the form of a directed relation, with a source locus, a target locus, and one direction, likewise does not tell you which directed relations a given system has. Someone has to look.

Consider three declared loci, written s, t, and u. As a matter of form alone, a directed relation could run between any ordered pair of them. Nothing in the form tells you which pairs are relations of this system. If Origin, from an observed distinction, declares the relation e = (s, t), then the directed difference operator gives

  A(x)[e] = x[s] − x[t]

on the numerical projections x[s] and x[t] produced by the declared M. The operator computes a value over the declared relation. It does not make (s, t) a relation, does not make (t, s) one, and says nothing about u. If no relation into or out of u has been declared, no operator says anything about u's relations. That is not because they are zero, but because they have not been declared: **not evaluated is not the same as evaluated with result 0.0** (`kernel_reference.md` §1).

This is why the canonical record can say that "numerical existence is not observational significance" (`kernel_reference.md` §1). A computation can be carried out on any relation anyone writes down. Whether it is about anything depends on whether the relation was declared from observation.

Readers trained in the abstract tradition may notice that this does not reject that tradition. Hilbert left it to the user of his axioms to say what the points are. Kernel V9 also leaves it to its user to say which relations hold, but it requires that step to be declared, traced to an observable, and recorded, and it builds the requirement into the operators and their implementation.

---

## 1.6 Responsibility: Origin and AI — [CANONICAL] with [EXPLANATORY] commentary

### The roles — [CANONICAL]

Kernel V9 holds that roles are not named into existence. They are defined by the functions performed (`role_separation_and_operator_application.md`).

| Role | Function | Held by |
|---|---|---|
| **Origin** | Declares D, M, the relations and their provenance, and what is admissible. Disposes of findings. Holds responsibility. | The human. Not assignable to a frame; cannot be delegated without being vacated. |
| **Generator** | Proposes candidate structure within what Origin declared. | Human or AI. |
| **Verifier** | Detects divergence from declared criteria, from a frame distinct from the Generator. Returns located findings, not approval. | Human or AI, from an independent frame. |

*Source: `kernel_reference.md` §3.*

### Origin's responsibility for observational declarations — [CANONICAL]

> "Origin — declaration and responsibility: declares D, M, the relations and their provenance, and what is admissible. … Declaration, admissibility, and disposition of findings remain with Origin throughout the declared workflow. *Origin declares.*" — `role_separation_and_operator_application.md`
>
> "Declaration is the act of stating traceability to an observable through M and taking responsibility for it." — `operators_notation_and_constraint.md`, Admissibility

Origin's declarations include (`kernel_reference.md` §5, Step 1): the domain D and mapping M; the relations, their source, and each one's single admissible direction; before persistence is evaluated, the sequence of actual observations and the direction of relational evolution, identified from an observable property of the process; and, for every analysis, the operator parameters ρ_base and χ₀ (and cc[p] for each declared component pair).

### The role of AI: implementing and executing authorized calculations — [CANONICAL]

In the canonical workflow, the Generator checks that the declaration is complete and then executes the applicable kernel unchanged (`kernel_reference.md` §5, Step 2):

> "If a required declaration is genuinely missing: stop at that exact missing declaration and return it to Origin. Do not substitute … If all required declarations exist: execute the applicable kernel (section 10) unchanged. State every projection with what it preserves and discards."

The execution discipline (`kernel_reference.md` §16.1) is:

```
Declare → Execute unchanged → Preserve output → Compare → Return correspondence/divergence to Origin
```

> "The Generator does not determine whether Origin's declaration is likely to succeed before executing it. … Neither participant may tune the declaration to obtain correspondence." — `kernel_reference.md` §16.1
>
> "Only Origin changes the declaration." — `kernel_reference.md` §16.5

### Commentary — [EXPLANATORY]

This division is not a statement about how capable an AI system is. It is a statement about where responsibility can sit. A calculation can be delegated, while answerability for what the calculation is about cannot. An AI system can implement the operators, run them over very large declared structures, keep exact records, and flag a missing declaration the moment execution reaches it. It cannot turn a declaration into an observation, and the framework does not let it try.

For the same reason, an AI system's output is checked for quantities that came from its training rather than from Origin's declaration (`kernel_reference.md` §20). A probability distribution, a default coupling constant, or a "mass" used as a primitive may each be familiar and may compute correctly. The question asked of each is the same: was it declared by Origin through M?

---

## 1.7 What this module does not claim — [EXPLANATORY]

- It does not claim that abstract mathematics is mistaken, or that its results are inadmissible within their own domains. The canonical rule concerns provenance within D: what may enter an ABR calculation, and how.
- It does not derive the choice of primitive. The canonical record states the primitive as "not derived. It is the starting point" (`kernel_reference.md` §1). This module reports that starting point. It does not prove it.
- It does not claim correspondence with any observed system. Correspondence is established by observable validation, or not at all (`role_separation_and_operator_application.md`, Verification and Validation Protocol).
- The historical account in §1.2 is a brief record from the general history of mathematics. It is not drawn from the canonical sources and carries no authority within the framework.

---

## 1.8 Reflection questions — [EXPLANATORY]

These are for discussion. They have no graded answers, and none of them asks for a proof.

1. Choose a physical law you know well. Which parts of it are mathematical structure, and which parts are the link between that structure and a measurement? Where in standard practice is that link recorded?
2. In set theory a relation is a set of ordered pairs of existing elements. What would have to be given first, before such a relation could be written down? Compare this with `kernel_reference.md` §1.
3. Why does the framework distinguish "not evaluated" from "evaluated with result 0.0"? Describe a situation in which confusing the two would lead to a wrong conclusion about a physical system.
4. In your own field, which judgements in an analysis could be handed to a computer, and which could not? Compare your answer with the role table in §1.6.

---

## Status summary for this module

| Section | Class | Canonical sources |
|---|---|---|
| 1.1 Where mathematics begins | [EXPLANATORY] | — |
| 1.2 The achievement of abstraction | [EXPLANATORY] | — (general history; see open question OQ-7) |
| 1.3 What abstraction sets aside | [EXPLANATORY], one [CANONICAL] quotation | `kernel_reference.md` §1 |
| 1.4 The observational starting point | [CANONICAL] | `kernel_reference.md` §1–§2; `operators_notation_and_constraint.md`; `abr_operators_plain.md` |
| 1.5 The central distinction | [CANONICAL], [EXPLANATORY] | `kernel_reference.md` §1, §9; `operators_notation_and_constraint.md` |
| 1.6 Responsibility: Origin and AI | [CANONICAL], [EXPLANATORY] | `kernel_reference.md` §3, §5, §16, §20; `role_separation_and_operator_application.md` |
| 1.7 What this module does not claim | [EXPLANATORY] | — |
| 1.8 Reflection questions | [EXPLANATORY] | — |

**[OPEN] items raised by this module:** see [`review/OPEN_QUESTIONS_FOR_ORIGIN.md`](../review/OPEN_QUESTIONS_FOR_ORIGIN.md), OQ-1, OQ-2, OQ-3, OQ-5, OQ-6, and OQ-7.

---

*Bounded over D. No claim beyond D.*
