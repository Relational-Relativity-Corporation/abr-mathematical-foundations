# 1 — The Basic Calculation (Exercise A)

Content classes: [CANONICAL] restates Kernel V9. [EXPLANATORY] is teaching material with no mathematical authority. [OPEN] is undeclared or undecided.

## 1.1 The calculation  [EXPLANATORY: Newtonian kinematics, not the ABR kernel]

A car travels at 60 mph on a level road. A hazard appears. After a reaction time the driver brakes, with constant deceleration, to a stop.

| Input | Value |
|---|---|
| Speed v | 60 mph = 26.8224 m/s |
| Reaction time t | 1.5 s |
| Braking deceleration a | 6.0 m/s², constant |
| Road | level |

```
d = v·t + v²/(2a)

reaction distance   v·t     = 26.8224 × 1.5        =  40.2336 m
braking distance    v²/(2a) = 719.44115 / 12.0     =  59.9534 m
stopping distance                                   = 100.1870 m   ≈ 100.2 m
```

Do this by hand first. Then run `cargo run`: the program prints the same three lines.

## 1.2 What the calculation assumes

1. The car is a point moving in a straight line.
2. The reaction time is a single fixed delay. In reality seeing the hazard, deciding, moving the foot and building up brake pressure are separate steps.
3. Deceleration jumps instantly to 6.0 m/s² and stays there. That one number stands in for the tyres, the road surface, its moisture, the brakes and ABS.
4. There is no grade, no wind, no air resistance and no rolling resistance.
5. Mass does not appear, because deceleration is given directly.
6. The speed is known exactly, and the hazard is seen the moment it appears.

## 1.3 What it can and cannot tell you

The arithmetic is exact *for its inputs*. Whether those inputs describe a particular stopping event is a separate question, and only observation answers it (Document 5).

The program runs the same assumptions through its reference simulator and gets 100.1870 m, a difference of +0.0000 m. That checks the simulator against the hand calculation. It says nothing about any real car.

*Bounded over D. No claim beyond D.*
