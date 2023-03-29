use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    let data = b"hello world";
    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).unwrap();
    assert_ne!(data, &enc_data[..]);

    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).unwrap();
    assert_eq!(data, &dec_data[..]);
}
