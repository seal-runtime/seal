<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

---

<h3>

```luau
yaml.encode: (t: YamlValue) -> string,
```

</h3>

---

<h3>

```luau
yaml.decode: (toml_data: string) -> YamlValue,
```

</h3>

---

<h3>

```luau
yaml.readfile: (path: string) -> YamlValue,
```

</h3>

---

<h3>

```luau
yaml.writefile: (path: string, content: YamlValue) -> (),
```

</h3>

---

---
