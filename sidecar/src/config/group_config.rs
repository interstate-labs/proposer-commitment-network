use std::{time::Duration, str::FromStr};
use alloy::primitives::b256;
/// Default slot time duration in seconds.
pub const DEFAULT_SLOT_TIME_SECONDS: u64 = 12;

/// Default commitment deadline duration.
pub const DEFAULT_COMMITMENT_DEADLINE_MILLIS: u64 = 8_000;

pub const HOLEKSY_CHAIN_ID:u64 = 17000;
pub const KURTOSIS_CHAIN_ID:u64 = 3151908;

/// Builder domain for signing messages on Holesky and Kurtosis.
const BUILDER_DOMAIN_HOLESKY: [u8; 32] = b256!("000000015b83a23759c560b2d0c64576e1dcfc34ea94c4988f3e0d9f77f05387").0;
const BUILDER_DOMAIN_KURTOSIS: [u8; 32] = b256!("000000010b41be4cdb34d183dddca5398337626dcdcfaf1720c1202d3b95f84e").0;

/// Chain configration
#[derive(Debug, Clone)]
pub struct ChainConfig {
    /// chain name
    pub(crate) chain: Chain,
    /// commitment deadline
    pub commitment_deadline: u64,
    /// customized slot time
    pub slot_time: u64,
    /// chain id
    pub id: u64,

}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain: Chain::Holesky,
            commitment_deadline: DEFAULT_COMMITMENT_DEADLINE_MILLIS,
            slot_time: DEFAULT_SLOT_TIME_SECONDS,
            id: HOLEKSY_CHAIN_ID
        }
    }
}

/// Available chains for the interstate sidecar
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Chain {
    Holesky,
    Kurtosis,
}

impl ChainConfig {
    /// get duration of commitment deadline.
    pub fn get_commitment_deadline_duration(&self) -> Duration {
        Duration::from_millis(self.commitment_deadline)
    }

    pub fn get_chain_id(&self) -> u64 {
        self.id
    }

    pub fn get_slot_time_in_seconds(&self) -> u64 {
        self.slot_time
    }

    /// Get the domain for signing messages on the given chain.
    pub fn builder_domain(&self) -> [u8; 32] {
        match self.chain {
            Chain::Holesky => BUILDER_DOMAIN_HOLESKY,
            Chain::Kurtosis => BUILDER_DOMAIN_KURTOSIS,
        }
    }

    
}

#[derive(Debug, Clone, Default)]
pub struct ValidatorIndexes(Vec<u64>);

impl ValidatorIndexes {
    pub fn contains(&self, index: u64) -> bool {
        self.0.contains(&index)
    }
}

impl FromStr for ValidatorIndexes {
    type Err = eyre::Report;

    /// Parse an array of validator indexes. Accepted values:
    /// - a single index (e.g. "1")
    /// - a comma-separated list of indexes (e.g. "1,2,3,4")
    /// - a contiguous range of indexes (e.g. "1..4")
    /// - a mix of the above (e.g. "1,2..4,6..8")
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut vec = Vec::new();

        for comma_separated_part in s.split(',') {
            if comma_separated_part.contains("..") {
                let mut parts = comma_separated_part.split("..");

                let start = parts.next().ok_or_else(|| eyre::eyre!("Invalid range"))?;
                let start = start.parse::<u64>()?;

                let end = parts.next().ok_or_else(|| eyre::eyre!("Invalid range"))?;
                let end = end.parse::<u64>()?;

                vec.extend(start..=end);
            } else {
                let index = comma_separated_part.parse::<u64>()?;
                vec.push(index);
            }
        }

        Ok(Self(vec))
    }
}

impl From<Vec<u64>> for ValidatorIndexes {
    fn from(vec: Vec<u64>) -> Self {
        Self(vec)
    }
}