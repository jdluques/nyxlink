use rand::{
    CryptoRng, RngCore,
    rngs::{OsRng, StdRng},
};

use crate::errors::internal::PrimitiveError;

#[derive(Debug, Clone, Copy)]
pub(crate) enum RandomnessSource {
    OS,
    Deterministic,
}

pub(crate) enum InnerRng {
    OS(OsRng),
    STD(StdRng),
}

pub(crate) struct SecureRng {
    inner: InnerRng,
    source: RandomnessSource,
}

impl SecureRng {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn fill(&mut self, out: &mut [u8]) -> Result<(), PrimitiveError> {
        self.try_fill_bytes(out)
            .map_err(|_| PrimitiveError::RandomnessUnavailable {
                source: self.source,
            })
    }
}

impl RngCore for SecureRng {
    fn next_u32(&mut self) -> u32 {
        match &mut self.inner {
            InnerRng::OS(rng) => rng.next_u32(),
            InnerRng::STD(rng) => rng.next_u32(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        match &mut self.inner {
            InnerRng::OS(rng) => rng.next_u64(),
            InnerRng::STD(rng) => rng.next_u64(),
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match &mut self.inner {
            InnerRng::OS(rng) => rng.fill_bytes(dest),
            InnerRng::STD(rng) => rng.fill_bytes(dest),
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        match &mut self.inner {
            InnerRng::OS(rng) => rng.try_fill_bytes(dest),
            InnerRng::STD(rng) => rng.try_fill_bytes(dest),
        }
    }
}

impl CryptoRng for SecureRng {}
