#include <stdio.h>
#include <string.h>

// Function for the Key Scheduling Algorithm (KSA)
void KSA(unsigned char S[], unsigned char key[], int key_len, int N) {
    int i, j;
    j = 0;
    for (i = 0; i < N; i++) {
        j = (j + S[i] + key[i % key_len]) % N;
        // Swap S[i] and S[j]
        unsigned char temp = S[i];
        S[i] = S[j];
        S[j] = temp;
    }
}

// Function for the Pseudo-Random Generation Algorithm (PGRA)
void PGRA(unsigned char S[], unsigned char data[], int data_len, int N) {
    int i, j;
    i = j = 0;
    for (int k = 0; k < data_len; k++) {
        i = (i + 1) % N;
        j = (j + S[i]) % N;
        // Swap S[i] and S[j]
        unsigned char temp = S[i];
        S[i] = S[j];
        S[j] = temp;
        int t = (S[i] + S[j]) % N;
        data[k] ^= S[t]; // XOR operation with the state vector
    }
}

int main() {
    unsigned char S[256]; // State vector for RC4
    unsigned char key[] = "SecretKey"; // Encryption key
    unsigned char input_text[] = "Hello, RC4!"; // Input text
    unsigned char ciphertext[256]; // Encrypted data (ciphertext)
    unsigned char decrypted_data[256]; // Decrypted data

    int N = 256; // Number of elements in the state vector

    int key_len = strlen(key);
    int text_len = strlen(input_text);

    printf("Input text: %s\n", input_text);
    printf("Key: %s\n\n", key);

    // Initialize S
    for (int i = 0; i < N; i++) {
        S[i] = i;
    }

    // Perform the Key Scheduling Algorithm (KSA) for both encryption and decryption
    KSA(S, key, key_len, N);

    // Perform encryption using PGRA
    PGRA(S, input_text, text_len, N);

    printf("Encryption complete.\n");

    // Display ciphertext
    printf("Ciphertext: ");
    for (int i = 0; i < text_len; i++) {
        printf("%02X ", input_text[i]);
    }
    printf("\n\n");

    // Reset S
    for (int i = 0; i < N; i++) {
        S[i] = i;
    }

    // Perform the Key Scheduling Algorithm (KSA) for decryption
    KSA(S, key, key_len, N);

    // Perform decryption using PGRA
    PGRA(S, input_text, text_len, N);

    // Display decrypted data
    printf("Decrypted data: %s\n", input_text);

    return 0;
}
