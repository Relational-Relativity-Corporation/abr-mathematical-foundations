//! Stopping-distance calculations.  [EXPLANATORY — NEWTONIAN, NOT KERNEL]
//!
//! Neither function here is a Kernel V9 operator, and neither is attributed to
//! the kernel. `basic` is Exercise A, exactly as Origin gave it. `expanded` is
//! the Generator-proposed Newtonian calculation of DESIGN_SPECIFICATION.md §6,
//! which is PENDING Origin authorization (D-SD-4): it is executed so that the
//! lesson can report what it gives, and every report labels it so.

#[derive(Clone, Debug)]
pub struct Basic {
    pub reaction_m: f64,
    pub braking_m: f64,
    pub total_m: f64,
}

/// d = v·t + v²/(2a).
pub fn basic(v: f64, t: f64, a: f64) -> Basic {
    let reaction_m = v * t;
    let braking_m = v * v / (2.0 * a);
    Basic { reaction_m, braking_m, total_m: reaction_m + braking_m }
}

/// One road segment between declared stations k and k+1.
#[derive(Clone, Debug)]
pub struct Segment {
    pub start_m: f64,
    pub length_m: f64,
    /// Uphill angle, radians (negative = downhill).
    pub angle: f64,
    /// Friction at the segment's start station, held over the segment.
    pub friction: f64,
}

/// Every input of the expanded calculation, with where it came from.
#[derive(Clone, Debug)]
pub struct ExpandedInputs {
    pub speed: f64,
    pub perception: f64,
    pub response: f64,
    pub buildup: f64,
    pub wind_along: f64,
    pub segments: Vec<Segment>,
    pub mass: f64,
    pub drag_area: f64,
    pub air_density: f64,
    pub g: f64,
}

#[derive(Clone, Debug)]
pub struct Expanded {
    pub reaction_m: f64,
    pub buildup_m: f64,
    pub braking_m: f64,
    pub total_m: f64,
}

fn seg_at(segs: &[Segment], x: f64) -> usize {
    segs.iter().rposition(|s| x >= s.start_m).unwrap_or(0)
}

/// Full-braking deceleration (positive) in a segment at entry speed v.
fn decel(inp: &ExpandedInputs, s: &Segment, v: f64) -> f64 {
    let rel = v - inp.wind_along;
    inp.g * (s.friction * s.angle.cos() + s.angle.sin())
        + 0.5 * inp.air_density * inp.drag_area * rel * rel.abs() / inp.mass
}

/// Reaction at constant speed; a linear build-up of deceleration to its
/// full-braking value; then constant deceleration per road segment, evaluated
/// at the speed with which the vehicle enters the segment.
/// Not modelled (deliberately; the reference simulator models them):
/// rolling resistance, drag varying within a segment, friction varying within
/// a segment, grade and drag during the reaction phase.
pub fn expanded(inp: &ExpandedInputs) -> Expanded {
    let reaction_m = inp.speed * (inp.perception + inp.response);
    let s0 = &inp.segments[seg_at(&inp.segments, reaction_m)];
    let a_full = decel(inp, s0, inp.speed);
    let tb = inp.buildup;
    // Linear ramp 0 → a_full over tb: Δx = v·tb − a_full·tb²/6, Δv = a_full·tb/2.
    let buildup_m = inp.speed * tb - a_full * tb * tb / 6.0;
    let mut v = inp.speed - a_full * tb / 2.0;
    let mut x = reaction_m + buildup_m;
    let start_braking = x;
    loop {
        let k = seg_at(&inp.segments, x);
        let s = &inp.segments[k];
        let a = decel(inp, s, v);
        let end = if k + 1 < inp.segments.len() { s.start_m + s.length_m } else { f64::INFINITY };
        let to_stop = v * v / (2.0 * a);
        if a > 0.0 && x + to_stop <= end {
            x += to_stop;
            break;
        }
        if !end.is_finite() {
            // Last segment and the vehicle cannot stop (a ≤ 0): no result.
            x = f64::NAN;
            break;
        }
        let dx = end - x;
        v = (v * v - 2.0 * a * dx).max(0.0).sqrt();
        x = end;
    }
    Expanded { reaction_m, buildup_m, braking_m: x - start_braking, total_m: x }
}
