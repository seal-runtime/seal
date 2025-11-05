type YamlValue = { [any]: any }

return _LOAD_EXTERN("@seal/std/serde/yaml") :: typeof({
	encode = function(t: YamlValue): string
		return nil :: any
	end,
	decode = function(toml_data: string): YamlValue
		return nil :: any
	end,
	readfile = function(path: string): YamlValue
		return nil :: any
	end,
	writefile = function(path: string, content: YamlValue): ()
		return nil :: any
	end
})