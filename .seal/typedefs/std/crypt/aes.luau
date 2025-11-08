export type aes = {
    generatekey: ()	-> string,
    encrypt: (plaintext: string, key: string) -> string,
    decrypt: (ciphertext: string, key: string) -> string,
}

return {} :: aes