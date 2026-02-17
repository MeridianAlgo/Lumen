use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
pub use ed25519_dalek::Error;

pub type KeyPair = SigningKey;
pub type PublicKey = VerifyingKey;
pub type Sig = Signature;

pub fn generate_keypair() -> KeyPair {
    let mut csprng = OsRng;
    SigningKey::generate(&mut csprng)
}

pub fn sign(keypair: &KeyPair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Signature) -> Result<(), Error> {
    public_key.verify(message, signature)
}
