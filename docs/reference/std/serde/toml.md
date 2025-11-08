<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.toml

`local toml = require("@std/serde/toml")`

Serialize TOML files.

---

<h3>

```luau
function toml.encode(t: TomlValue) -> string,
```

</h3>

---

<h3>

```luau
function toml.decode(toml_data: string) -> TomlValue,
```

</h3>

---

<h3>

```luau
function toml.readfile(path: string) -> TomlValue,
```

</h3>

---

<h3>

```luau
function toml.writefile(path: string, content: TomlValue) -> (),
```

</h3>

---

## `export type` TomlValue

---
