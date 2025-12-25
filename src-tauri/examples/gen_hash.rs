use ai_lot_lib::infrastructure::crypto::CryptoService;

fn main() {
    let password = "admin123";
    match CryptoService::hash_password(password) {
        Ok(hash) => {
            println!("Password: {}", password);
            println!("Hash: {}", hash);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
