use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TickDirection {
    ZeroPlusTick,
    ZeroMinusTick,
    PlusTick,
    MinusTick,
}

#[cfg(test)]
mod tests_tick_direction {
    use super::*;

    #[test]
    fn deserlise() {
        assert_eq!(
            serde_json::from_value::<TickDirection>(json!("MinusTick")).unwrap(),
            TickDirection::MinusTick
        )
    }
}
