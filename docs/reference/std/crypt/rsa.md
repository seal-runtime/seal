<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

---

### RsaKeys.public

<h4>

```luau
RsaKeys.public: string,
```

</h4>

---

### RsaKeys.private

<h4>

```luau
RsaKeys.private: string
```

</h4>

---

## `export type` Rsa

---

### Rsa.generatekeys

<h4>

```luau
Rsa.generatekeys: () -> RsaKeys,
```

</h4>

---

### Rsa.encrypt

<h4>

```luau
Rsa.encrypt: (plaintext: string, public_key: string) -> string,
```

</h4>

---

### Rsa.decrypt

<h4>

```luau
Rsa.decrypt: (ciphertext: string, private_key: string) -> string,
```

</h4>

---
