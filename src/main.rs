use base64::{engine::general_purpose, Engine as _};
use des::TdesEde3;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;

fn main() {
    // Replace with your values
    let base64_key = "YOUR_BASE64_KEY_HERE";
    let base64_encrypted = "YOUR_BASE64_ENCRYPTED_DATA";

    // Decode Base64 inputs
    let key_bytes = general_purpose::STANDARD.decode(base64_key).expect("Invalid Base64 key");
    let encrypted_bytes = general_purpose::STANDARD.decode(base64_encrypted).expect("Invalid Base64 data");

    // ECB mode for 3DES
    type TdesEcb = Ecb<TdesEde3, Pkcs7>;
    let cipher = TdesEcb::new_from_slices(&key_bytes, &[]).expect("Invalid key");

    let decrypted_data = cipher.decrypt_vec(&encrypted_bytes).expect("Decryption failed");
    let decrypted_str = String::from_utf8(decrypted_data).expect("Invalid UTF-8");

    println!("Decrypted Trade JSON data: {}", decrypted_str);
}
