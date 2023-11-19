use std::io;

// Function for the Key Scheduling Algorithm (KSA)
fn ksa(s: &mut [u8; 256], key: &[u8], key_len: usize, n: usize) {
    let mut j = 0;
    for i in 0..n {
        j = (j + s[i] as usize + key[i % key_len] as usize) % n;
        // Swap s[i] and s[j]
        s.swap(i, j);
    }
}

// Function for the Pseudo-Random Generation Algorithm (PGRA)
fn pgra(s: &mut [u8; 256], data: &mut [u8], data_len: usize, n: usize) {
    let mut i = 0;
    let mut j = 0;
    for k in 0..data_len {
        i = (i + 1) % n;
        j = (j + s[i] as usize) % n;
        // Swap s[i] and s[j]
        s.swap(i, j);
        let t = (s[i] as usize + s[j] as usize) % n;
        data[k] ^= s[t]; // XOR operation with the state vector
    }
}

fn main() {
    let mut s: [u8; 256] = [0; 256]; // State vector for RC4
    let key: [u8; 9] = *b"SecretKey"; // Encryption key
    let mut input_text: Vec<u8> = "Hello, RC4!".as_bytes().to_vec(); // Input text
    let mut ciphertext: Vec<u8> = vec![0; input_text.len()]; // Encrypted data (ciphertext)
    let mut decrypted_data: Vec<u8> = vec![0; input_text.len()]; // Decrypted data

    let n = 256; // Number of elements in the state vector

    let key_len = key.len();
    let text_len = input_text.len();

    println!("Input text: {}", String::from_utf8_lossy(&input_text));
    println!("Key: {}\n", String::from_utf8_lossy(&key));

    // Initialize S
    for i in 0..n {
        s[i] = i as u8;
    }

    // Perform the Key Scheduling Algorithm (KSA) for both encryption and decryption
    ksa(&mut s, &key, key_len, n);

    // Perform encryption using PGRA
    pgra(&mut s, &mut input_text, text_len, n);

    println!("Encryption complete.");

    // Display ciphertext
    print!("Ciphertext: ");
    for byte in &input_text {
        print!("{:02X} ", byte);
    }
    println!("\n");

    // Reset S
    for i in 0..n {
        s[i] = i as u8;
    }

    // Perform the Key Scheduling Algorithm (KSA) for decryption
    ksa(&mut s, &key, key_len, n);

    // Perform decryption using PGRA
    pgra(&mut s, &mut input_text, text_len, n);

    // Display decrypted data
    println!("Decrypted data: {}", String::from_utf8_lossy(&input_text));
}
