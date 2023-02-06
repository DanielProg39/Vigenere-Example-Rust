# Vigenère Cipher Example for Rust
Simple code that implements basic Vigenère Cipher alghoritm in Rust

------------
### How is it built?
It is built using a simple pseudo-code example for implementing Vigenère cipher alghoritm. You can see it below:
```
m <-- length of key
for index, character in plaintext:
    ciphertext[index] <-- (character + key[index % m]) % 26
return ciphertext
```
However, it is made more complex in order to not encrypt the spaces and by doing so, to make encryption of whole sentences possible. Since the v0.2.0 decryption is also supported.
### Limitations
This program is a very simple example of Vigenère cipher alghoritm, so it takes only uppercase English letters and spaces for plaintext or ciphertext and uppercase English letters for a key. Any other symbol will not be encrypted correctly.
### The output
```
Plaintext: HELLO WORLD
Key: KEY
Ciphertext: RIJVS UYVJN
Decrypted ciphertext: HELLO WORLD
```
To change the values and the output, clone this repo and change the main function as you want in main.rs.
