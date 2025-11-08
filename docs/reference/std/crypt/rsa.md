<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.rsa

`local rsa = require("@std/crypt/rsa")`

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

## `export type` Rsa

---

<h3>

```luau
function Rsa.generatekeys() -> RsaKeys,
```

</h3>

---

<h3>

```luau
function Rsa.encrypt(plaintext: string, public_key: string) -> string,
```

</h3>

---

<h3>

```luau
function Rsa.decrypt(ciphertext: string, private_key: string) -> string,
```

</h3>

---
