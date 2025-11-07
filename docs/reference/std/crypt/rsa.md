<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

RsaKeys.public: `string`

RsaKeys.private: `string`

`export type` Rsa

Rsa.generatekeys: `() -> RsaKeys`

Rsa.encrypt: `(plaintext: string, public_key: string) -> string`

Rsa.decrypt: `(ciphertext: string, private_key: string) -> string`
