use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, TryUnwrap)]
#[serde(untagged)]
pub enum LinearTickerData {
    Snapshot(LinearTickerDataSnapshot),
    Delta(LinearTickerDataDelta),
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::fixture;

    use super::*;

    #[test]
    fn deserialize() {
        let json = fixture!("ws_linear_ticker");
        let values = serde_json::from_str::<Vec<WsTicker>>(json)
            .unwrap()
            .into_iter()
            .map(|t| t.data.try_unwrap_linear().unwrap())
            .collect_vec();
        assert_eq!(values.len(), 102);
    }
}
