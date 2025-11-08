<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

---

### yaml.encode

<h4>

```luau
function (t: YamlValue) -> string,
```

</h4>

---

### yaml.decode

<h4>

```luau
function (toml_data: string) -> YamlValue,
```

</h4>

---

### yaml.readfile

<h4>

```luau
function (path: string) -> YamlValue,
```

</h4>

---

### yaml.writefile

<h4>

```luau
function (path: string, content: YamlValue) -> (),
```

</h4>

---

## `export type` YamlValue

---
