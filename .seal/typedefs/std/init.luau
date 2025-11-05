--[=[
Of course *you'd* require the whole standard library at once...
]=]
local std = {
	fs = require("@self/fs"),
	str = require("@self/str"),
	semver = require("@self/semver"),
	env = require("@self/env"),
	io = require("@self/io"),
	colors = require("@self/colors"),
	format = require("@self/io/format"),
	time = require("@self/time"),
	datetime = require("@self/time/datetime"),
	process = require("@self/process"),
	serde = require("@self/serde"),
	json = require("@self/json"),
	net = require("@self/net"),
	thread = require("@self/thread"),
	luau = require("@self/luau"),
}

return std