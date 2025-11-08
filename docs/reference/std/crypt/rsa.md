<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

---

<h3>
```luau
RsaKeys.public: string,
```
</h3>

---

<h3>
```luau
RsaKeys.private: string
```
</h3>

---

---

<h3>
```luau
Rsa.generatekeys: () -> RsaKeys,
```
</h3>

---

<h3>
```luau
Rsa.encrypt: (plaintext: string, public_key: string) -> string,
```
</h3>

---

<h3>
```luau
Rsa.decrypt: (ciphertext: string, private_key: string) -> string,
```
</h3>

---
