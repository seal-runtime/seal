-- requiring std libraries works (it's an alias ig)
local fs = require("@std/fs")
local format = require("@std/io/format")

local function aliased()
	local relative_but_not_relatively = (require :: any)("@tests/luau/globals/require/basic-requires/relative_data")
	assert(relative_but_not_relatively.relative == "yes very", "alias required file returned unexpected result")
end

aliased()

local function unabletoextractalias()
	local s, f = pcall(function()
		local impossible = (require)("@/hi")
		return nil
	end)
	assert(s == false, "extracting alias was not supposed to succeed")
	assert(tostring(f):match("couldn't extract require alias"), "received different error when trying to extract alias")
end

unabletoextractalias()

local function nonexistentalias()
	local s, f = pcall(function()
		local impossible = (require)("@hi/hi")
		return nil
	end)
	assert(s == false, "requiring nonexisting alias was not supposed to succeed")
	assert(format.uncolor(tostring(f)):match("alias 'hi' not found in the %.luaurc"), "unexpected alias not found message")
end

nonexistentalias()

local function aliaswithoutslashfile()
	local process_values = (require :: any)("@output_formatter")
	assert(process_values.simple ~= nil, "output_formatter.simple_print should be non-nil")
end

aliaswithoutslashfile()

local function aliaswithoutslashdir()
	local data_path = fs.path.join(".", "tests", "data")
	local data_dir = fs.dir.from(data_path)
		:add_file("init.luau", "return 'successful'")
	
	local res = (require :: any)("@data")
	assert(res == "successful", "why is requiring @data not successful?")
	fs.removefile(data_dir:join("init.luau"))
end

aliaswithoutslashdir()

local function aliaswithfullnamespecs()
	local data_path = fs.path.join(".", "tests", "data")
	local data_dir = fs.dir.from(data_path)
		:add_file("init.luau", "return 'successful'")
	
	local res = (require)("@seal.gamertag-epic_swag-hina-_seal.level-69420_1337.requires-tests.data_directory")
	assert(res == "successful", "requiring with full alias name specs failed. please double-check impl or alias")
	fs.removefile(data_dir:join("init.luau"))
end

aliaswithfullnamespecs()

local function aliaswithredundantfileextension()
	local s, f = pcall(function()
		require("@redundant")
		return nil
	end)
	assert(s == false, "this should not work - alias path to file cannot have redundant `.luau`. should only work when it is a directory")
	-- FIXME: missing assertion for expected error message (not impl yet)
	
	local path = fs.path.join(".", ".seal", "extra")
	local newdir_path = fs.path.join(path, "newdir.luau")
	local newdir = fs.tree()
			:with_file("init.luau", "return \"successful\"")
			:with_file("main.luau", "return \"failure\"")
	fs.writetree(newdir_path, newdir)
	
	local s2, f2 = pcall(function()
		return (require)("@redundant-but-dir")
	end)
	fs.removetree(newdir_path)
	
	assert(s2 == true, "this should work - alias path to directory can have redundant `.luau`")
	assert(rawequal(f2, "successful"), "incorrectly required directory")
end

aliaswithredundantfileextension()
