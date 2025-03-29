use crate::DecodeError::CryptoError;
use crate::{DecodeError, TlsResult};
use ring::agreement::{EphemeralPrivateKey, PublicKey};
use ring::{agreement, rand};

// https://docs.rs/ring/latest/ring/agreement/index.html
pub fn create_paired_keys() -> TlsResult<(EphemeralPrivateKey, PublicKey), DecodeError> {
    let rnd = rand::SystemRandom::new();
    let priv_key = EphemeralPrivateKey::generate(&agreement::X25519, &rnd)
        .map_err(|_x| CryptoError("cannot generate agreement of X25519".into()))?;
    let pub_key = priv_key
        .compute_public_key()
        .map_err(|_x| CryptoError("cannot compute public key of X25519".into()))?;

    Ok((priv_key, pub_key))
}

pub fn generate_peer_key(
    peer_pub_key: &[u8; 32],
    priv_key: EphemeralPrivateKey,
) -> TlsResult<(), DecodeError> {
    // let peer_public_key_bytes = {
    //     // In a real application, the peer public key would be parsed out of a
    //     // protocol message. Here we just generate one.
    //     let peer_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    //     peer_private_key.compute_public_key()?
    // };
    // let pp = PublicKey::try_from(peer_pub_key)?;
    let peer_public_key = agreement::UnparsedPublicKey::new(&agreement::X25519, peer_pub_key);

    let _x = agreement::agree_ephemeral(priv_key, &peer_public_key, |_key_material| {
        // In a real application, we'd apply a KDF to the key material and the
        // public keys (as recommended in RFC 7748) and then derive session
        // keys from the result. We omit all that here.
    })
    .map_err(|_x| CryptoError("cannot generate peer key of X25519".into()))?;

    Ok(())
}

pub fn create_random_u8_32() -> [u8; 32] {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut buf = [0_u8; 32];
    for i in 0..32_usize {
        seed ^= seed.rotate_left(13);
        buf[i] = seed as u8;
        if buf[i] == 0 {
            buf[i] = 128;
        }
    }

    buf
}
