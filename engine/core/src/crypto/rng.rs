#[cfg(test)]
use rand::rngs::StdRng;
use rand_core::{OsRng, TryRngCore};

use crate::errors::internal::PrimitiveError;

#[derive(Debug, Clone, Copy)]
pub enum RandomnessSource {
    OS,
    Deterministic,
}

pub enum InnerRng {
    OS(OsRng),
    #[cfg(test)]
    STD(StdRng),
}

pub struct SecureRng {
    inner: InnerRng,
    source: RandomnessSource,
}

impl SecureRng {
    pub fn new() -> Self {
        #[cfg(not(test))]
        {
            Self {
                inner: InnerRng::OS(OsRng),
                source: RandomnessSource::OS,
            }
        }

        #[cfg(test)]
        {
            use rand::SeedableRng;

            let mut seed_bytes = [0u8; 32];
            seed_bytes[0..8].copy_from_slice(&42u64.to_le_bytes());
            Self {
                inner: InnerRng::STD(StdRng::from_seed(seed_bytes)),
                source: RandomnessSource::Deterministic,
            }
        }
    }

    pub fn fill(&mut self, out: &mut [u8]) -> Result<(), PrimitiveError> {
        match &mut self.inner {
            InnerRng::OS(rng) => fill_helper(rng, out, self.source),
            #[cfg(test)]
            InnerRng::STD(rng) => fill_helper(rng, out, self.source),
        }
    }
}

fn fill_helper<R: TryRngCore>(
    rng: &mut R,
    buf: &mut [u8],
    source: RandomnessSource,
) -> Result<(), PrimitiveError> {
    rng.try_fill_bytes(buf)
        .map_err(|_| PrimitiveError::RandomnessUnavailable { source })
}
