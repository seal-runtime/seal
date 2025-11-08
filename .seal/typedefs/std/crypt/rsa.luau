export type rsa = {
    generatekeys: () -> RsaKeys,
    encrypt: (plaintext: string, public_key: string) -> string,
    decrypt: (ciphertext: string, private_key: string) -> string,
}

type RsaKeys = {
    public: string,
    private: string
}

return {} :: rsa