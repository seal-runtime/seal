<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.hash

`local hash = require("@std/crypt/hash")`

Contains function sha2, which can be used to create an **unsalted** hash, returned as a buffer.

### Please use the `@std/crypt/password` library if you want to hash passwords (salted)

---

### hash.sha2

<h4>

```luau
function hash.sha2(plaintext: string) -> buffer,
```

</h4>

Hashes plaintext with the SHA2-256 algorithm, returns a buffer (of length 32) containing the result.

You can use the @std/serde library to convert the result into a 'readable' format

---
