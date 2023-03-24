wai_bindgen_rust::export!("universe.wai");
use secret_toolkit_crypto::sha256;
pub struct Universe;

impl universe::Universe for Universe {
    fn hash(input: String) -> String {
        base64::encode(sha256(input.as_bytes()))
    }
}