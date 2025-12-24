use zeroize::{Zeroize, Zeroizing};

use crate::{
    errors::CoreError,
    memory::{
        lock::{self, LockStrategy},
        zeroize::ZeroizePolicy,
    },
};

pub struct SecretVector {
    buf: Zeroizing<Vec<u8>>,
    lock_strategy: LockStrategy,
    zeroize_policy: ZeroizePolicy,
}

impl SecretVector {
    #[allow(private_interfaces)]
    pub fn new(
        bytes: &[u8],
        lock_strategy: LockStrategy,
        zeroize_policy: ZeroizePolicy,
    ) -> Result<Self, CoreError> {
        let mut buf = Zeroizing::new(bytes.to_vec());
        lock::lock_memory(&mut buf, lock_strategy)?;
        Ok(Self {
            buf,
            lock_strategy,
            zeroize_policy,
        })
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }

    pub fn as_ref(&self) -> &[u8] {
        &self.buf
    }

    pub fn zeroize(&mut self) {
        if self.zeroize_policy == ZeroizePolicy::Explicit {
            self.buf.zeroize();
        } else {
            log::warn!("Attempted to zeroize buffer with Non-Explicit zeroize policy");
        }
    }
}

impl Drop for SecretVector {
    fn drop(&mut self) {
        if self.lock_strategy != LockStrategy::None {
            lock::unlock_memory(&mut self.buf);
        }
        self.buf.zeroize();
    }
}
