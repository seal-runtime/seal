<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.toml

`local toml = require("@std/serde/toml")`

Serialize TOML files.

---

## toml.encode

<h4>

```luau
toml.encode: (t: TomlValue) -> string,
```

</h4>

---

## toml.decode

<h4>

```luau
toml.decode: (toml_data: string) -> TomlValue,
```

</h4>

---

## toml.readfile

<h4>

```luau
toml.readfile: (path: string) -> TomlValue,
```

</h4>

---

## toml.writefile

<h4>

```luau
toml.writefile: (path: string, content: TomlValue) -> (),
```

</h4>

---

## `export type` TomlValue

---
