<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.base64

`local base64 = require("@std/serde/base64")`

good for serving binary stuff in a digestable form for serving things on the internet

---

### base64.encode

<h4>

```luau
function (data: string | buffer) -> string,
```

</h4>

---

### base64.decode

<h4>

```luau
function (data: string) -> buffer,
```

</h4>

---

### base64.urlsafe.encode

<h4>

```luau
function (data: string | buffer) -> string,
```

</h4>

---

### base64.urlsafe.decode

<h4>

```luau
function (data: string) -> buffer
```

</h4>

---
