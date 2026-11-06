//! Exercise C reference simulator.  [EXPLANATORY — NOT KERNEL]
//!
//! A deterministic, fixed-step, single-track longitudinal model of one braking
//! event. It is the *reference observation* of this lesson. Kernel V9 admits a
//! simulated observation only as simulated (v9_specification.md, Simulated
//! observations): nothing here is empirical evidence.
//!
//! Declared mechanics (Newtonian; every value comes from
//! declarations/simulator_illustrative.json):
//!   gravity along the grade        −g·sinθ(x)
//!   normal force                   N = m·g·cosθ(x)
//!   rolling resistance             −C_rr·N
//!   aerodynamic drag               −½·ρ_air·C_dA·(v − w)·|v − w|
//!   braking (tyre-limited, ABS)    −b(t)·μ(x)·N, with b(t) the brake fraction:
//!                                  0 until perception + response, then a
//!                                  linear ramp to 1 over the build-up time
//! Friction μ(x) and elevation z(x) are interpolated linearly between road
//! stations. Tread surface speed ωr = v·(1 − s_peak·b(t)).
//!
//! The simulator deliberately models effects the expanded calculation
//! (calc.rs) does not (design D-SD-9): rolling resistance, drag varying
//! continuously with speed, friction varying within a station segment, and
//! grade and drag acting during the reaction phase.

use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub mass: f64,
    pub drag_area: f64,
    pub air_density: f64,
    pub crr: f64,
    pub g: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Station {
    pub position_m: f64,
    pub elevation_m: f64,
    pub friction: f64,
}

#[derive(Clone, Debug)]
pub struct Trial {
    pub id: String,
    pub speed: f64,
    pub perception: f64,
    pub response: f64,
    pub buildup: f64,
    pub wind_speed: f64,
    /// Direction the wind blows toward, measured from the direction of travel.
    pub wind_direction_deg: f64,
    pub peak_slip: f64,
    pub stations: Vec<Station>,
}

impl Trial {
    /// Wind velocity component along the direction of travel (+ = tailwind).
    /// This is the declared M projection for the Air locus (DESIGN D-SD-3).
    pub fn wind_along(&self) -> f64 {
        self.wind_speed * self.wind_direction_deg.to_radians().cos()
    }
}

/// Readings the simulated instruments deliver (the reference observation).
#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub trial: String,
    pub mode: String,
    /// F1 at hazard onset (t = 0): air (along-road), body, tread, road.
    pub onset_velocity_m_s: [f64; 4],
    /// F1 at the instant full braking is reached.
    pub full_braking_velocity_m_s: [f64; 4],
    pub full_braking_time_s: f64,
    /// F3 event times from hazard onset: obstacle visible, driver response
    /// onset, brake pressure onset, full deceleration.
    pub event_times_s: [f64; 4],
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    pub stations: Vec<Station>,
    /// Reference stopping distance: position at standstill − position at
    /// hazard onset.
    pub stopping_distance_m: f64,
    pub stopping_time_s: f64,
    /// The vehicle's observed states: hazard onset, response start, brake
    /// onset, full braking, standstill. Position along the road from the
    /// hazard-onset position (m) and speed (m/s) at each.
    pub state_positions_m: [f64; 5],
    pub state_speeds_m_s: [f64; 5],
    /// Trajectory sampled every 0.1 s: (t, x, v, deceleration).
    pub trajectory: Vec<[f64; 4]>,
}

fn interp(stations: &[Station], x: f64, f: impl Fn(&Station) -> f64) -> f64 {
    let n = stations.len();
    if x <= stations[0].position_m {
        return f(&stations[0]);
    }
    for k in 0..n - 1 {
        let (a, b) = (&stations[k], &stations[k + 1]);
        if x <= b.position_m {
            let t = (x - a.position_m) / (b.position_m - a.position_m);
            return f(a) + t * (f(b) - f(a));
        }
    }
    f(&stations[n - 1])
}

fn slope(stations: &[Station], x: f64) -> f64 {
    let n = stations.len();
    let k = (0..n - 1).find(|&k| x < stations[k + 1].position_m).unwrap_or(n - 2);
    let (a, b) = (&stations[k], &stations[k + 1]);
    ((b.elevation_m - a.elevation_m) / (b.position_m - a.position_m)).atan()
}

fn brake_fraction(tr: &Trial, t: f64) -> f64 {
    let onset = tr.perception + tr.response;
    if t < onset {
        0.0
    } else if tr.buildup <= 0.0 || t >= onset + tr.buildup {
        1.0
    } else {
        (t - onset) / tr.buildup
    }
}

/// Full declared mechanics.
pub fn simulate(tr: &Trial, veh: &Vehicle, dt: f64) -> Observation {
    let w = tr.wind_along();
    let t_full = tr.perception + tr.response + tr.buildup;
    let (mut t, mut x, mut v) = (0.0_f64, 0.0_f64, tr.speed);
    let mut traj = Vec::new();
    let mut next_sample = 0.0;
    let mut full_v = None;
    let state_times = [0.0, tr.perception, tr.perception + tr.response, t_full];
    let mut states: Vec<(f64, f64)> = Vec::new();
    loop {
        while states.len() < 4 && t + 1e-12 >= state_times[states.len()] {
            states.push((x, v));
        }
        let th = slope(&tr.stations, x);
        let mu = interp(&tr.stations, x, |s| s.friction);
        let n = veh.mass * veh.g * th.cos();
        let b = brake_fraction(tr, t);
        let rel = v - w;
        let force = -veh.mass * veh.g * th.sin()
            - veh.crr * n
            - 0.5 * veh.air_density * veh.drag_area * rel * rel.abs()
            - b * mu * n;
        let acc = force / veh.mass;
        if t + 1e-12 >= next_sample {
            traj.push([round6(t), round6(x), round6(v), round6(-acc)]);
            next_sample += 0.1;
        }
        if full_v.is_none() && t + 1e-12 >= t_full {
            full_v = Some(v);
        }
        let v_new = v + acc * dt;
        if v_new <= 0.0 {
            // Standstill within this step: advance to v = 0 at constant acc.
            let dt_stop = -v / acc;
            x += v * dt_stop + 0.5 * acc * dt_stop * dt_stop;
            t += dt_stop;
            break;
        }
        x += 0.5 * (v + v_new) * dt;
        v = v_new;
        t += dt;
    }
    let vf = full_v.unwrap_or(0.0);
    while states.len() < 4 { states.push((x, 0.0)); }
    let state_positions_m = [states[0].0, states[1].0, states[2].0, states[3].0, x];
    let state_speeds_m_s = [states[0].1, states[1].1, states[2].1, states[3].1, 0.0];
    Observation {
        trial: tr.id.clone(),
        mode: "full declared mechanics".into(),
        onset_velocity_m_s: [w, tr.speed, tr.speed, 0.0],
        full_braking_velocity_m_s: [w, vf, vf * (1.0 - tr.peak_slip), 0.0],
        full_braking_time_s: t_full,
        event_times_s: [0.0, tr.perception, tr.perception + tr.response, t_full],
        wind_speed_m_s: tr.wind_speed,
        wind_direction_deg: tr.wind_direction_deg,
        stations: tr.stations.clone(),
        stopping_distance_m: x,
        stopping_time_s: t,
        state_positions_m,
        state_speeds_m_s,
        trajectory: traj,
    }
}

/// The Exercise A assumptions, run through the same integrator: constant
/// speed for the reaction time, then a constant deceleration step. Used only
/// to check the simulator against the hand calculation (conformance of the
/// simulator, not physical accuracy).
pub fn simulate_basic(speed: f64, reaction: f64, decel: f64, dt: f64) -> f64 {
    let (mut t, mut x, mut v) = (0.0_f64, 0.0_f64, speed);
    loop {
        let acc = if t + 1e-12 < reaction { 0.0 } else { -decel };
        let v_new = v + acc * dt;
        if v_new <= 0.0 {
            let dt_stop = -v / acc;
            x += v * dt_stop + 0.5 * acc * dt_stop * dt_stop;
            break;
        }
        x += 0.5 * (v + v_new) * dt;
        v = v_new;
        t += dt;
    }
    x
}

fn round6(v: f64) -> f64 {
    (v * 1e6).round() / 1e6
}
