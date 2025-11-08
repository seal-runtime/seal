<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

---

### RsaKeys.public

```luau
RsaKeys.public: string,
```

---

### RsaKeys.private

```luau
RsaKeys.private: string
```

---

### `export type` Rsa

---

### Rsa.generatekeys

```luau
Rsa.generatekeys: () -> RsaKeys,
```

---

### Rsa.encrypt

```luau
Rsa.encrypt: (plaintext: string, public_key: string) -> string,
```

---

### Rsa.decrypt

```luau
Rsa.decrypt: (ciphertext: string, private_key: string) -> string,
```

---
