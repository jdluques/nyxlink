use crate::crypto::algorithms;

pub(crate) fn hash_with_domain(domain: &[u8], data: &[u8], hash_alg: algorithms::Hash) -> [u8; 64] {
    use sha2::Digest;

    match hash_alg {
        algorithms::Hash::SHA512 => {
            use sha2::Sha512;
            let mut h = Sha512::new();
            h.update(domain);
            h.update(&[0u8]);
            h.update(data);
            h.finalize().into()
        }
    }
}
