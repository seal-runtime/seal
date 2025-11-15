export type aes = {
    --- Generate an AES-256 key.
    generatekey: ()	-> string,
    --- Encrypt plaintext with an AES-256 key
    encrypt: (plaintext: string, key: string) -> string,
    --- Decrypt ciphertext with an AES-256 key.
    decrypt: (ciphertext: string, key: string) -> string,
}

return {} :: aes