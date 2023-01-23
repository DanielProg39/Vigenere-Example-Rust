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
However, it is made more complex in order to not encrypt the spaces and by doing so, to make encryption of whole sentences possible.
### Limitations
This program is a very simple example of Vigenère cipher alghoritm, so it takes only uppercase English letters and spaces for plaintext and uppercase English letters for a key. Any other symbol will not be encrypted correctly.
