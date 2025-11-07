<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.yaml

`local yaml = require("@std/serde/yaml")`

Serialize YAML files.

yaml.encode: `(t: YamlValue) -> string`

yaml.decode: `(toml_data: string) -> YamlValue`

yaml.readfile: `(path: string) -> YamlValue`

yaml.writefile: `(path: string, content: YamlValue) -> ()`

YamlValue
