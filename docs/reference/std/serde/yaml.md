<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

---

<h3>

```luau
function yaml.encode(t: YamlValue) -> string,
```

</h3>

---

<h3>

```luau
function yaml.decode(toml_data: string) -> YamlValue,
```

</h3>

---

<h3>

```luau
function yaml.readfile(path: string) -> YamlValue,
```

</h3>

---

<h3>

```luau
function yaml.writefile(path: string, content: YamlValue) -> (),
```

</h3>

---

## `export type` YamlValue

---
