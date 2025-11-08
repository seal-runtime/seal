<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

---

### rsa.generatekeys

<h4>

```luau
function rsa.generatekeys() -> RsaKeys,
```

</h4>

---

### rsa.encrypt

<h4>

```luau
function rsa.encrypt(plaintext: string, public_key: string) -> string,
```

</h4>

---

### rsa.decrypt

<h4>

```luau
function rsa.decrypt(ciphertext: string, private_key: string) -> string,
```

</h4>

---

## `export type` RsaKeys

---

### RsaKeys.public

<h4>

```luau
public: string,
```

</h4>

---

### RsaKeys.private

<h4>

```luau
private: string
```

</h4>

---
