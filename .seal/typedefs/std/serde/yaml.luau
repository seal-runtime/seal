--[=[
Serialize YAML files.
]=]
export type yaml = {
    encode: (t: YamlValue) -> string,
    decode: (toml_data: string) -> YamlValue,
    readfile: (path: string) -> YamlValue,
    writefile: (path: string, content: YamlValue) -> (),
}

type YamlValue = { [any]: any }

return {} :: yaml