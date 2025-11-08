<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.toml

`local toml = require("@std/serde/toml")`

Serialize TOML files.

---

toml.encode: `(t: TomlValue) -> string`

---

toml.decode: `(toml_data: string) -> TomlValue`

---

toml.readfile: `(path: string) -> TomlValue`

---

toml.writefile: `(path: string, content: TomlValue) -> ()`

---

`export type` TomlValue

---
