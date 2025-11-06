<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Rsa

`RsaKeys.public: string`

`RsaKeys.private: string`

`export type Rsa =`

`Rsa.generatekeys: () -> RsaKeys`

`Rsa.encrypt: (plaintext: string, public_key: string) -> string`

`Rsa.decrypt: (ciphertext: string, private_key: string) -> string`
