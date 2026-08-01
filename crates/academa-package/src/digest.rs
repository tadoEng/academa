use sha2::{Digest, Sha256};

pub(crate) fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

/// Computes the lowercase SHA-256 transport digest of exact package bytes.
///
/// This digest identifies a candidate ZIP byte stream. It is not a locally issued
/// publication or lesson-version identity.
#[must_use]
pub fn compute_package_digest(package_bytes: &[u8]) -> String {
    sha256_digest(package_bytes)
}
