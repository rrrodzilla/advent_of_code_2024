use rayon::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

// 🌉 Engineering measurements and calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeStress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadFactor(pub u64);

// 🐘 Operators stolen by playful elephants
#[derive(Debug, Clone, Copy)]
pub enum JungleOperator {
    Sum,        // + (Addition operator found in northern jungle)
    Product,    // * (Multiplication operator found in western jungle)
    StringJoin, // || (Concatenation operator hidden in dense undergrowth)
}

impl JungleOperator {
    // 🔧 Apply engineering operations to structural components
    pub fn apply(&self, left_load: u64, right_load: u64) -> Option<u64> {
        match self {
            JungleOperator::Sum => left_load.checked_add(right_load),
            JungleOperator::Product => left_load.checked_mul(right_load),
            JungleOperator::StringJoin => {
                // Fast path for common cases to avoid string allocation
                if right_load < 10 {
                    return left_load.checked_mul(10)?.checked_add(right_load);
                }
                if right_load < 100 {
                    return left_load.checked_mul(100)?.checked_add(right_load);
                }

                // Fall back to string handling for larger numbers
                let right_digits = right_load.to_string();
                let decimal_shift = 10u64.checked_pow(right_digits.len() as u32)?;
                left_load
                    .checked_mul(decimal_shift)?
                    .checked_add(right_load)
            }
        }
    }
}

// 🚧 Calibration system error handling
#[derive(Debug)]
pub enum BridgeError {
    InvalidMeasurement(ParseIntError),
    MalformedEquation,
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::InvalidMeasurement(e) => write!(f, "Invalid stress measurement: {}", e),
            BridgeError::MalformedEquation => write!(f, "Malformed calibration equation"),
        }
    }
}

impl std::error::Error for BridgeError {}

// 📐 Single bridge calibration equation
#[derive(Debug)]
pub struct BridgeEquation {
    pub target_stress: BridgeStress,
    pub load_factors: Vec<LoadFactor>,
}

impl FromStr for BridgeEquation {
    type Err = BridgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(':').collect();
        if components.len() != 2 {
            return Err(BridgeError::MalformedEquation);
        }

        let target_stress = components[0]
            .trim()
            .parse()
            .map(BridgeStress)
            .map_err(BridgeError::InvalidMeasurement)?;

        let load_factors = components[1]
            .split_whitespace()
            .map(|n| n.parse().map(LoadFactor))
            .collect::<Result<Vec<_>, _>>()
            .map_err(BridgeError::InvalidMeasurement)?;

        Ok(BridgeEquation {
            target_stress,
            load_factors,
        })
    }
}

impl BridgeEquation {
    // 🎯 Check if equation can reach target stress with available operators
    pub fn is_calibratable(&self, allow_string_join: bool) -> bool {
        let factors = &self.load_factors;
        let len = factors.len();

        // 🚫 Bridge needs at least two support points
        if len < 2 {
            return false;
        }

        // 🌿 Quick check for common cases
        match len {
            2 => {
                let (a, b) = (factors[0].0, factors[1].0);
                // Check sum and product first (most common)
                if a + b == self.target_stress.0 || a * b == self.target_stress.0 {
                    return true;
                }
                // Only try string join if needed
                allow_string_join
                    && JungleOperator::StringJoin.apply(a, b) == Some(self.target_stress.0)
            }
            3 => {
                let (a, b, c) = (factors[0].0, factors[1].0, factors[2].0);
                // Check most common patterns first
                if a + b * c == self.target_stress.0 || a * b + c == self.target_stress.0 {
                    return true;
                }
                if !allow_string_join {
                    return false;
                }
                // Only check string join combinations if needed
                for op1 in [
                    JungleOperator::Sum,
                    JungleOperator::Product,
                    JungleOperator::StringJoin,
                ] {
                    if let Some(ab) = op1.apply(a, b) {
                        for op2 in [
                            JungleOperator::Sum,
                            JungleOperator::Product,
                            JungleOperator::StringJoin,
                        ] {
                            if op2.apply(ab, c) == Some(self.target_stress.0) {
                                return true;
                            }
                        }
                    }
                }
                false
            }
            _ => {
                let operator_slots = len - 1;
                let possible_combinations = if allow_string_join {
                    3u64.pow(operator_slots as u32) // Three operators available
                } else {
                    2u64.pow(operator_slots as u32) // Only + and * available
                };

                // 🦁 Parallel jungle search for valid operator combinations
                (0..possible_combinations).into_par_iter().any(|combo| {
                    let mut current_stress = factors[0].0;

                    // 🔄 Apply operators in sequence
                    for i in 0..operator_slots {
                        let operator = if allow_string_join {
                            // Base-3: Sum = 0, Product = 1, StringJoin = 2
                            match (combo / 3u64.pow(i as u32)) % 3 {
                                0 => JungleOperator::Sum,
                                1 => JungleOperator::Product,
                                _ => JungleOperator::StringJoin,
                            }
                        } else {
                            // Base-2: Sum = 0, Product = 1
                            if (combo >> i) & 1 == 0 {
                                JungleOperator::Sum
                            } else {
                                JungleOperator::Product
                            }
                        };

                        if let Some(next_stress) = operator.apply(current_stress, factors[i + 1].0)
                        {
                            current_stress = next_stress;
                        } else {
                            return false;
                        }
                    }

                    current_stress == self.target_stress.0
                })
            }
        }
    }
}

// 🏗️ Bridge calibration system
pub struct BridgeCalibrator {
    equations: Vec<BridgeEquation>,
}

impl BridgeCalibrator {
    // 📋 Load calibration data from engineering notes
    pub fn new(input: &str) -> Result<Self, BridgeError> {
        let equations = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(BridgeCalibrator { equations })
    }

    // 🔍 Calculate total stress with basic operators
    pub fn calculate_basic_stress(&self) -> u64 {
        self.equations
            .iter()
            .filter(|eq| eq.is_calibratable(false))
            .map(|eq| eq.target_stress.0)
            .sum()
    }

    // 🌿 Calculate total stress including hidden jungle operator
    pub fn calculate_advanced_stress(&self) -> u64 {
        self.equations
            .iter()
            .filter(|eq| eq.is_calibratable(true))
            .map(|eq| eq.target_stress.0)
            .sum()
    }
}

#[cfg(test)]
mod structural_tests {
    use super::*;

    const SAMPLE_SPECS: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn verify_basic_stress() {
        let calibrator = BridgeCalibrator::new(SAMPLE_SPECS).unwrap();
        assert_eq!(calibrator.calculate_basic_stress(), 3749);
    }

    #[test]
    fn verify_advanced_stress() {
        let calibrator = BridgeCalibrator::new(SAMPLE_SPECS).unwrap();
        assert_eq!(calibrator.calculate_advanced_stress(), 11387);
    }
}
