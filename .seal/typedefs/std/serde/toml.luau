--[=[
Serialize TOML files.
]=]
export type toml = {
    encode: (t: TomlValue) -> string,
    decode: (toml_data: string) -> TomlValue,
    readfile: (path: string) -> TomlValue,
    writefile: (path: string, content: TomlValue) -> (),
}

export type TomlValue = { [any]: any }

return {} :: toml