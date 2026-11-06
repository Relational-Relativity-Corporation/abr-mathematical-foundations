# 4 — Interdependent Observable Conditions

**The point of this document: the significance of one observable condition can depend on its relationships with other observable conditions.**

Content classes are as in [Document 1](01_BASIC_CALCULATION.md).

## 4.1 Design

There are four controlled trials for each pair of conditions. Everything else is held at the base trial:
- same vehicle, 60 mph unless speed is the factor;
- perception 0.8 s, response 0.45 s, build-up 0.25 s;
- calm air, level road, good tread, dry surface.

Each trial runs through the reference simulator, its five fields go through IRRS and the kernel, and the basic and expanded calculations are made. The four reference stopping distances then form one field F4, a **diamond** of declared relations:

```
        a0_b0 ──(factor a)──▶ a1_b0
          │                     │
     (factor b)            (factor b)
          ▼                     ▼
        a0_b1 ──(factor a)──▶ a1_b1
```

The interior loci are a1_b0 and a0_b1. The diamond is acyclic, so it is not a ring.

A on a "factor a" edge is the difference in stopping distance when only factor a changes, with factor b held at one level. **If the two "factor a" edges carry different A values, the significance of factor a depends on factor b.** That conclusion comes from setting two admitted A outputs side by side. **No interaction formula is applied anywhere**, and the combined effect is not assumed to be the sum of the separate effects.

[OPEN] These relations connect *separate trials*, not an observed passage. Whether a controlled-condition contrast is an admissible declared relation is Origin's decision (D-SD-8). IRRS evaluated every diamond as OPEN (15 conditions) with 0 violations.

## 4.2 Where each interaction comes from  [read this before the results]

| Diamond | Source of any interaction |
|---|---|
| tread × moisture | **A table chosen by the Generator**: one friction value per trial (good/dry 0.85, worn/dry 0.80, good/wet 0.55, worn/wet 0.35). The interaction shown is the one written into the table. The lesson shows how such an interaction *appears*. It does not establish that real tyres behave this way |
| visibility × attention | **A table chosen by the Generator**: one perception delay per trial (0.8, 1.3, 1.1 and 2.0 s). The same caution applies |
| wind × speed | Nothing tabled. It comes from the simulator's declared drag law: force ∝ (air speed relative to the car)² |
| grade × surface | Nothing tabled. It comes from the declared mechanics: gravity along the grade, and friction ∝ the normal force |

## 4.3 Results (reference simulated distances; −A = increase in stopping distance)

| Diamond | Change in factor a, at b = level 0 | Change in factor a, at b = level 1 | Change in factor b, at a = level 0 | Change in factor b, at a = level 1 |
|---|---|---|---|---|
| tread (good → worn) × moisture (dry → wet) | dry: **+2.486 m** | wet: **+33.550 m** | good: +21.443 m | worn: +52.508 m |
| attention (attentive → distracted) × illumination (day → dusk) | day: +12.712 m | dusk: +22.758 m | attentive: +7.635 m | distracted: +17.681 m |
| wind (calm → 15 m/s headwind) × speed (60 → 80 mph) | 60 mph: −2.223 m | 80 mph: −3.991 m | calm: +43.081 m | headwind: +41.313 m |
| grade (level → −6 %) × surface (dry → wet) | dry: +6.318 m | wet: +12.187 m | level: +21.443 m | downgrade: +27.313 m |

**Reading the tread row.** On a dry road, worn tread adds about 2.5 m. On a wet road it adds about 33.6 m. The two A values differ, so in this declared data the significance of tread depends on moisture.

**Reading the wind row.** A headwind *shortens* the stop, because it adds drag. It shortens it more at 80 mph than at 60 mph. That interaction was not tabled: it follows from the declared drag law.

**What the basic calculation shows.** Its inputs never change, so it gives 100.187 m in all sixteen trials. It cannot register any of these conditions.

Per-trial basic, expanded and reference distances are in `records/LESSON1_REPORT.txt`, and the field digests for each trial are in `records/diamonds/trials_summary.txt`.

*Bounded over D. No claim beyond D.*
