<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

---

## yaml.encode

<h4>

```luau
yaml.encode: (t: YamlValue) -> string,
```

</h4>

---

## yaml.decode

<h4>

```luau
yaml.decode: (toml_data: string) -> YamlValue,
```

</h4>

---

## yaml.readfile

<h4>

```luau
yaml.readfile: (path: string) -> YamlValue,
```

</h4>

---

## yaml.writefile

<h4>

```luau
yaml.writefile: (path: string, content: YamlValue) -> (),
```

</h4>

---

## `export type` YamlValue

---
