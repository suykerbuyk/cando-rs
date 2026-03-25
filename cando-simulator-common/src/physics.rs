//! Physics simulation utilities for realistic device behavior.
//!
//! This module provides reusable physics simulation functions for modeling
//! realistic device behavior in CAN simulators, including:
//! - Smooth transitions (exponential and linear ramping)
//! - Thermal modeling (heat generation and cooling)
//! - Hysteresis-based limiting
//!
//! # Examples
//!
//! ```rust
//! use cando_simulator_common::physics::{exponential_ramp, thermal_model};
//!
//! // Motor speed ramping
//! let mut current_rpm = 0.0;
//! let target_rpm = 1000.0;
//! let time_constant = 2.0; // seconds
//! let dt = 0.1; // 100ms update rate
//!
//! current_rpm = exponential_ramp(current_rpm, target_rpm, time_constant, dt);
//!
//! // Temperature simulation
//! let mut temp = 25.0;
//! let power = 100.0; // 100W heat generation
//! temp = thermal_model(temp, 25.0, power, 0.5, 50.0, dt);
//! ```

/// Exponential ramping for smooth transitions.
///
/// Models first-order system dynamics with exponential approach to target value.
/// This produces natural-looking behavior for motors, temperatures, and other
/// physical systems that don't change instantaneously.
///
/// # Mathematical Model
///
/// Uses exponential decay formula:
/// ```text
/// x(t+dt) = x(t) + a * (target - x(t))
/// where a = 1 - exp(-dt / tau)
/// ```
///
/// # Arguments
///
/// * `current` - Current value
/// * `target` - Target value to approach
/// * `time_constant` - Time constant tau (seconds). Larger values = slower response.
///   After 1*tau, reaches ~63% of target. After 5*tau, reaches ~99% of target.
/// * `dt` - Time step (seconds)
///
/// # Returns
///
/// New value after one time step
///
/// # Examples
///
/// ## Motor Speed Control
///
/// ```rust
/// use cando_simulator_common::physics::exponential_ramp;
///
/// let mut rpm = 0.0;
/// let target_rpm = 1000.0;
/// let time_constant = 2.0; // 2 second time constant
/// let dt = 0.1; // 100ms update rate
///
/// // Simulate 10 steps (1 second)
/// for _ in 0..10 {
///     rpm = exponential_ramp(rpm, target_rpm, time_constant, dt);
/// }
///
/// // After 1 second (0.5*tau), should be around 40% of target
/// assert!(rpm > 300.0 && rpm < 500.0);
/// ```
///
/// ## Temperature Settling
///
/// ```rust
/// use cando_simulator_common::physics::exponential_ramp;
///
/// let mut temp = 100.0; // Start hot
/// let ambient = 25.0;
/// let cooling_time_constant = 5.0; // 5 seconds to cool
/// let dt = 0.1;
///
/// // Cool for 10 seconds (2*tau)
/// for _ in 0..100 {
///     temp = exponential_ramp(temp, ambient, cooling_time_constant, dt);
/// }
///
/// // After 2*tau, reaches ~86% of target, so ~36 degrees remaining
/// assert!(temp < 40.0 && temp > 30.0);
/// ```
pub fn exponential_ramp(current: f64, target: f64, time_constant: f64, dt: f64) -> f64 {
    // Calculate alpha (step size) from time constant
    let alpha = 1.0 - (-dt / time_constant).exp();

    // Apply exponential approach
    current + alpha * (target - current)
}

/// Linear ramping with rate limit.
///
/// Approaches target value at a fixed maximum rate, useful for slew-rate limited
/// systems like motor controllers, valve actuators, or other systems with
/// maximum rate-of-change constraints.
///
/// # Arguments
///
/// * `current` - Current value
/// * `target` - Target value to approach
/// * `max_rate` - Maximum change rate per second (absolute value)
/// * `dt` - Time step (seconds)
///
/// # Returns
///
/// New value after one time step, limited by max_rate
///
/// # Examples
///
/// ## Motor Controller with Slew Rate Limit
///
/// ```rust
/// use cando_simulator_common::physics::linear_ramp;
///
/// let mut rpm = 0.0;
/// let target_rpm = 1000.0;
/// let max_rate = 500.0; // 500 RPM/sec max acceleration
/// let dt = 0.1; // 100ms update
///
/// // First step
/// rpm = linear_ramp(rpm, target_rpm, max_rate, dt);
/// assert_eq!(rpm, 50.0); // 500 RPM/s * 0.1s = 50 RPM
///
/// // Continue ramping
/// for _ in 0..19 {
///     rpm = linear_ramp(rpm, target_rpm, max_rate, dt);
/// }
///
/// // After 2 seconds, should reach target
/// assert_eq!(rpm, 1000.0);
/// ```
///
/// ## Bidirectional Rate Limiting
///
/// ```rust
/// use cando_simulator_common::physics::linear_ramp;
///
/// let mut position = 100.0;
/// let target = 0.0;
/// let max_rate = 50.0; // 50 units/sec
/// let dt = 0.1;
///
/// // Ramp down
/// position = linear_ramp(position, target, max_rate, dt);
/// assert_eq!(position, 95.0); // Decreased by 5.0
/// ```
pub fn linear_ramp(current: f64, target: f64, max_rate: f64, dt: f64) -> f64 {
    let delta = target - current;
    let max_change = max_rate * dt;

    // Clamp change to maximum rate
    current + delta.clamp(-max_change, max_change)
}

/// Simple thermal model with heat generation and ambient cooling.
///
/// Models temperature dynamics as a first-order system with:
/// - Heat generation from power dissipation (P * R_th)
/// - Exponential cooling to ambient temperature
/// - Thermal capacitance determining response time
///
/// # Mathematical Model
///
/// ```text
/// Steady-state temperature rise: dT = P * R_th
/// Target temperature: T_target = T_ambient + dT
/// Time constant: tau = R_th * C_th
/// Temperature dynamics: First-order exponential approach
/// ```
///
/// # Arguments
///
/// * `current_temp` - Current temperature (degrees C)
/// * `ambient_temp` - Ambient temperature (degrees C)
/// * `power` - Heat generation power (W). Use 0 for pure cooling.
/// * `thermal_resistance` - Thermal resistance R_th (degrees C/W). Higher = slower cooling.
/// * `thermal_capacitance` - Thermal capacitance C_th (J/degrees C). Higher = more thermal mass.
/// * `dt` - Time step (seconds)
///
/// # Returns
///
/// New temperature after one time step (degrees C)
///
/// # Examples
///
/// ## Motor Heating Under Load
///
/// ```rust
/// use cando_simulator_common::physics::thermal_model;
///
/// let mut temp = 25.0; // Start at ambient
/// let power = 100.0; // 100W heat generation
/// let r_th = 0.5; // 0.5 degrees C/W thermal resistance
/// let c_th = 50.0; // 50 J/degrees C thermal capacitance
/// let dt = 1.0; // 1 second update
///
/// // Heat up for 60 seconds
/// for _ in 0..60 {
///     temp = thermal_model(temp, 25.0, power, r_th, c_th, dt);
/// }
///
/// // Steady state: 25 + 100W * 0.5 = 75 degrees C
/// // After 60 seconds (~2.4*tau), should be close
/// assert!(temp > 65.0 && temp < 75.0);
/// ```
///
/// ## Cooling After Power Off
///
/// ```rust
/// use cando_simulator_common::physics::thermal_model;
///
/// let mut temp = 75.0; // Start hot
/// let power = 0.0; // Power off, just cooling
/// let r_th = 0.5;
/// let c_th = 50.0;
/// let dt = 1.0;
///
/// // Cool for 30 seconds
/// for _ in 0..30 {
///     temp = thermal_model(temp, 25.0, power, r_th, c_th, dt);
/// }
///
/// // Should have cooled significantly
/// assert!(temp < 50.0);
/// ```
///
/// ## Variable Load Simulation
///
/// ```rust
/// use cando_simulator_common::physics::thermal_model;
///
/// let mut temp = 25.0;
/// let r_th = 0.5;
/// let c_th = 50.0;
/// let dt = 1.0;
///
/// // High load for 10 seconds
/// for _ in 0..10 {
///     temp = thermal_model(temp, 25.0, 200.0, r_th, c_th, dt);
/// }
/// let temp_high_load = temp;
///
/// // Low load for 10 seconds
/// for _ in 0..10 {
///     temp = thermal_model(temp, 25.0, 50.0, r_th, c_th, dt);
/// }
///
/// // Temperature should decrease with lower load
/// assert!(temp < temp_high_load);
/// ```
pub fn thermal_model(
    current_temp: f64,
    ambient_temp: f64,
    power: f64,
    thermal_resistance: f64,
    thermal_capacitance: f64,
    dt: f64,
) -> f64 {
    // Calculate steady-state temperature rise from power dissipation
    let temp_rise = power * thermal_resistance;
    let target_temp = ambient_temp + temp_rise;

    // Time constant = R * C (thermal RC time constant)
    let tau = thermal_resistance * thermal_capacitance;

    // Use exponential ramp to approach steady-state
    exponential_ramp(current_temp, target_temp, tau, dt)
}

/// Clamp value to range with hysteresis.
///
/// Prevents oscillation around limit boundaries by requiring the value to
/// move away from the limit by a hysteresis amount before unclamping.
/// Useful for over-temperature protection, over-current limiting, etc.
///
/// # Arguments
///
/// * `value` - Current value to clamp
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
/// * `hysteresis` - Deadband distance from limit
/// * `previously_clamped` - Whether value was clamped in previous step
///
/// # Returns
///
/// Tuple of `(clamped_value, is_clamped)`
///
/// # Examples
///
/// ## Over-Temperature Protection
///
/// ```rust
/// use cando_simulator_common::physics::clamp_with_hysteresis;
///
/// let max_temp = 100.0;
/// let hysteresis = 5.0;
/// let mut was_limited = false;
///
/// // Temperature rises to limit
/// let temp = 101.0;
/// let (clamped, limited) = clamp_with_hysteresis(temp, 0.0, max_temp, hysteresis, was_limited);
/// assert_eq!(clamped, max_temp); // Clamped to 100
/// assert!(limited); // Now in limited state
/// was_limited = limited;
///
/// // Still above limit (but within hysteresis)
/// let temp = 99.0;
/// let (clamped, limited) = clamp_with_hysteresis(temp, 0.0, max_temp, hysteresis, was_limited);
/// assert_eq!(clamped, max_temp); // Still clamped (within hysteresis)
/// assert!(limited);
/// was_limited = limited;
///
/// // Below hysteresis threshold - unlocks
/// let temp = 94.0;
/// let (clamped, limited) = clamp_with_hysteresis(temp, 0.0, max_temp, hysteresis, was_limited);
/// assert_eq!(clamped, 94.0); // No longer clamped
/// assert!(!limited); // Unlocked
/// ```
///
/// ## Prevents Oscillation
///
/// ```rust
/// use cando_simulator_common::physics::clamp_with_hysteresis;
///
/// let max = 10.0;
/// let hysteresis = 2.0;
///
/// // Without hysteresis, value would oscillate around 10.0
/// // With hysteresis, it stays clamped until it drops to 8.0
///
/// let mut was_limited = false;
/// let mut value = 11.0;
///
/// // Clamp
/// (value, was_limited) = clamp_with_hysteresis(value, 0.0, max, hysteresis, was_limited);
/// assert_eq!(value, 10.0);
/// assert!(was_limited);
///
/// // Small decrease - still clamped (prevents oscillation)
/// value = 9.5;
/// (value, was_limited) = clamp_with_hysteresis(value, 0.0, max, hysteresis, was_limited);
/// assert_eq!(value, 10.0); // Still clamped
/// assert!(was_limited);
/// ```
pub fn clamp_with_hysteresis(
    value: f64,
    min: f64,
    max: f64,
    hysteresis: f64,
    previously_clamped: bool,
) -> (f64, bool) {
    if previously_clamped {
        // Currently in clamped state - need to move away from limit to unclamp
        if value < min + hysteresis {
            (min, true) // Still at lower limit
        } else if value > max - hysteresis {
            (max, true) // Still at upper limit
        } else {
            (value, false) // Moved away from limit, unclamp
        }
    } else {
        // Not currently clamped - normal clamping
        if value < min {
            (min, true) // Hit lower limit
        } else if value > max {
            (max, true) // Hit upper limit
        } else {
            (value, false) // Within range
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Exponential ramp tests
    // ========================================================================

    #[test]
    fn test_exponential_ramp_approaches_target() {
        let mut value = 0.0;
        let target = 100.0;
        let tau = 1.0;
        let dt = 0.1;

        // After 1 time constant, should reach ~63% of target
        for _ in 0..10 {
            value = exponential_ramp(value, target, tau, dt);
        }
        assert!(value > 60.0 && value < 66.0);
    }

    #[test]
    fn test_exponential_ramp_never_overshoots() {
        let mut value = 0.0;
        let target = 100.0;
        let tau = 1.0;
        let dt = 0.1;

        for _ in 0..1000 {
            value = exponential_ramp(value, target, tau, dt);
            assert!(value <= target + 0.01); // Never overshoot
        }
    }

    #[test]
    fn test_exponential_ramp_bidirectional() {
        let mut value = 100.0;
        let target = 0.0;
        let tau = 1.0;
        let dt = 0.1;

        for _ in 0..10 {
            value = exponential_ramp(value, target, tau, dt);
        }

        // Should decrease toward target
        assert!(value < 40.0);
        assert!(value >= 0.0);
    }

    #[test]
    fn test_exponential_ramp_slow_vs_fast() {
        let mut slow = 0.0;
        let mut fast = 0.0;
        let target = 100.0;
        let dt = 0.1;

        // Slow response (large time constant)
        slow = exponential_ramp(slow, target, 10.0, dt);

        // Fast response (small time constant)
        fast = exponential_ramp(fast, target, 1.0, dt);

        // Fast should change more in same time
        assert!(fast > slow);
    }

    // ========================================================================
    // Linear ramp tests
    // ========================================================================

    #[test]
    fn test_linear_ramp_constant_rate() {
        let mut value = 0.0;
        let target = 1000.0;
        let max_rate = 100.0;
        let dt = 0.1;

        // Each step should increase by exactly 10.0
        for _ in 0..10 {
            let prev = value;
            value = linear_ramp(value, target, max_rate, dt);
            assert!((value - prev - 10.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_linear_ramp_reaches_target() {
        let mut value = 0.0;
        let target = 100.0;
        let max_rate = 50.0;
        let dt = 1.0;

        // Should take exactly 2 steps
        value = linear_ramp(value, target, max_rate, dt);
        assert_eq!(value, 50.0);

        value = linear_ramp(value, target, max_rate, dt);
        assert_eq!(value, 100.0);
    }

    #[test]
    fn test_linear_ramp_bidirectional() {
        let mut value = 100.0;
        let target = 0.0;
        let max_rate = 50.0;
        let dt = 1.0;

        value = linear_ramp(value, target, max_rate, dt);
        assert_eq!(value, 50.0); // Decreased by 50
    }

    #[test]
    fn test_linear_ramp_at_target() {
        let value = 100.0;
        let target = 100.0;
        let max_rate = 50.0;
        let dt = 1.0;

        let result = linear_ramp(value, target, max_rate, dt);
        assert_eq!(result, 100.0); // No change
    }

    // ========================================================================
    // Thermal model tests
    // ========================================================================

    #[test]
    fn test_thermal_model_steady_state() {
        let mut temp = 25.0;
        let power = 100.0;
        let r_th = 0.5; // Should reach 25 + 100*0.5 = 75 degrees C
        let c_th = 10.0;
        let dt = 1.0;

        // Run for many time constants
        for _ in 0..500 {
            temp = thermal_model(temp, 25.0, power, r_th, c_th, dt);
        }

        // Should be very close to steady state
        assert!((temp - 75.0).abs() < 1.0);
    }

    #[test]
    fn test_thermal_model_cooling() {
        let mut temp = 100.0;
        let power = 0.0; // No heat generation
        let r_th = 0.5;
        let c_th = 10.0;
        let dt = 1.0;

        // Cool for some time
        for _ in 0..50 {
            temp = thermal_model(temp, 25.0, power, r_th, c_th, dt);
        }

        // Should have cooled toward ambient
        assert!(temp < 100.0);
        assert!(temp > 25.0);
    }

    #[test]
    fn test_thermal_model_never_negative() {
        let mut temp = 0.0;
        let power = 0.0;
        let r_th = 1.0;
        let c_th = 1.0;
        let dt = 1.0;

        for _ in 0..100 {
            temp = thermal_model(temp, 25.0, power, r_th, c_th, dt);
            assert!(temp >= 0.0);
        }
    }

    // ========================================================================
    // Hysteresis clamp tests
    // ========================================================================

    #[test]
    fn test_hysteresis_normal_clamping() {
        let (clamped, limited) = clamp_with_hysteresis(150.0, 0.0, 100.0, 5.0, false);
        assert_eq!(clamped, 100.0);
        assert!(limited);
    }

    #[test]
    fn test_hysteresis_stays_clamped() {
        // Already clamped at upper limit
        let (clamped, limited) = clamp_with_hysteresis(99.0, 0.0, 100.0, 5.0, true);
        assert_eq!(clamped, 100.0); // Still clamped
        assert!(limited);
    }

    #[test]
    fn test_hysteresis_releases_below_threshold() {
        // Value drops below (max - hysteresis)
        let (clamped, limited) = clamp_with_hysteresis(94.0, 0.0, 100.0, 5.0, true);
        assert_eq!(clamped, 94.0); // Released
        assert!(!limited);
    }

    #[test]
    fn test_hysteresis_lower_limit() {
        // Test lower limit with hysteresis
        let (clamped, limited) = clamp_with_hysteresis(-10.0, 0.0, 100.0, 5.0, false);
        assert_eq!(clamped, 0.0);
        assert!(limited);

        // Within hysteresis - stays clamped
        let (clamped, limited) = clamp_with_hysteresis(3.0, 0.0, 100.0, 5.0, limited);
        assert_eq!(clamped, 0.0);
        assert!(limited);

        // Beyond hysteresis - releases
        let (clamped, limited) = clamp_with_hysteresis(6.0, 0.0, 100.0, 5.0, limited);
        assert_eq!(clamped, 6.0);
        assert!(!limited);
    }

    #[test]
    fn test_hysteresis_within_range() {
        let (clamped, limited) = clamp_with_hysteresis(50.0, 0.0, 100.0, 5.0, false);
        assert_eq!(clamped, 50.0);
        assert!(!limited);
    }

    #[test]
    fn test_hysteresis_prevents_oscillation() {
        let mut value = 101.0;
        let mut was_limited = false;

        // Hit limit
        (value, was_limited) = clamp_with_hysteresis(value, 0.0, 100.0, 5.0, was_limited);
        assert_eq!(value, 100.0);
        assert!(was_limited);

        // Small decrease - still limited (prevents oscillation)
        (value, was_limited) = clamp_with_hysteresis(99.0, 0.0, 100.0, 5.0, was_limited);
        assert_eq!(value, 100.0); // Still clamped
        assert!(was_limited);

        // Large decrease - releases
        (value, was_limited) = clamp_with_hysteresis(94.0, 0.0, 100.0, 5.0, was_limited);
        assert_eq!(value, 94.0);
        assert!(!was_limited);
    }
}
