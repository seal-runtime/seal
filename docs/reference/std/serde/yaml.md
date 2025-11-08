<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

---

### yaml.encode

```luau
yaml.encode: (t: YamlValue) -> string,
```

---

### yaml.decode

```luau
yaml.decode: (toml_data: string) -> YamlValue,
```

---

### yaml.readfile

```luau
yaml.readfile: (path: string) -> YamlValue,
```

---

### yaml.writefile

```luau
yaml.writefile: (path: string, content: YamlValue) -> (),
```

---

### `export type` YamlValue

```luau

```

---
