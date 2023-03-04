use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GokurakuConfig {}

#[derive(Debug)]
pub struct GokurakuConfigInstance {}

impl TryFrom<GokurakuConfig> for GokurakuConfigInstance {
    type Error = anyhow::Error;

    fn try_from(_value: GokurakuConfig) -> Result<Self, Self::Error> {
        Ok(GokurakuConfigInstance {})
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
