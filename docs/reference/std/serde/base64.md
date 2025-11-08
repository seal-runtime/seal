<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.base64

`local base64 = require("@std/serde/base64")`

good for serving binary stuff in a digestable form for serving things on the internet

---

<h3>

```luau
base64.encode: (data: string | buffer) -> string,
```

</h3>

---

<h3>

```luau
base64.decode: (data: string) -> buffer,
```

</h3>

---

<h3>

```luau
base64.urlsafe.encode: (data: string | buffer) -> string,
```

</h3>

---

<h3>

```luau
base64.urlsafe.decode: (data: string) -> buffer
```

</h3>

---
