--[=[
	Standard library for handling **terminal** input/output.

	You can require the whole `@std/io` lib at once but it's recommended you require individual
	libraries as you need them instead.

	- To read input from the terminal, use `@std/io/prompt` or if you need lower-level control, `@std/io/input`.
	- To write directly to stdout/stderr, use `@std/io/output`.
	- Want to format your tables in the same pretty way as `print`? Use `@std/io/format`.
	- If you want to be colorful, use `@std/io/colors`, which is aliased to just `@std/colors` as well (due to high traffic).
]=]
local io = {
	input = require("@self/input"),
	output = require("@self/output"),
	colors = require("@self/colors"),
	format = require("@self/format"),
	prompt = require("@self/prompt"),
}

return io