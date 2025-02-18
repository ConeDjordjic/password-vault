use rand::{thread_rng, Rng};

pub fn generate_password(length: usize, special_chars: bool) -> String {
    let mut rng = thread_rng();
    let mut password = String::with_capacity(length);
    
    let chars = if special_chars {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()"
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    };
    
    for _ in 0..length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars.chars().nth(idx).unwrap());
    }
    
    password
}
