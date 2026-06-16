-- Credit to Bottersnike

local function isProbablyUrl(x: string)
	return x:sub(1, 7) == "http://" or x:sub(1, 8) == "https://"
end
local function isProbablyGitUrl(x: string)
	if isProbablyUrl(x) then
		return true
	end
	return x:sub(1, 6) == "git://" or x:sub(1, 6) == "ssh://"
end

-- Validation for _RUNTIME
if not _RUNTIME then
	error("_RUNTIME not defined")
end
if type(_RUNTIME) ~= "table" then
	error("_RUNTIME must be present and a table")
end
if not table.isfrozen(_RUNTIME) then
	error("_RUNTIME must be frozen")
end
if type(_RUNTIME.name) ~= "string" then
	error("_RUNTIME.name must be present and a string")
end
if _RUNTIME.name ~= string.lower(_RUNTIME.name) then
	error("_RUNTIME.name must be strictly lower-case")
end
if type(_RUNTIME.version) ~= "table" then
	error("_RUNTIME.version must be present and a table")
end
if not table.isfrozen(_RUNTIME.version) then
	error("_RUNTIME.version must be frozen")
end
if type(_RUNTIME.version.display) ~= "string" then
	error("_RUNTIME.version.display must be present and a string")
end
if _RUNTIME.version.semantic ~= nil then
	if type(_RUNTIME.version.semantic) ~= "table" then
		error("when present, _RUNTIME.version.semantic must be a table")
	end
	if not table.isfrozen(_RUNTIME.version.semantic) then
		error("when present, _RUNTIME.version.semantic must be frozen")
	end

	if type(_RUNTIME.version.semantic.major) ~= "number" then
		error("when .semantic present, _RUNTIME.version.semantic.major must be present and a number")
	end
	if type(_RUNTIME.version.semantic.minor) ~= "number" then
		error("when .semantic present, _RUNTIME.version.semantic.minor must be present and a number")
	end
	if type(_RUNTIME.version.semantic.patch) ~= "number" then
		error("when .semantic present, _RUNTIME.version.semantic.patch must be present and a number")
	end

	if _RUNTIME.version.semantic.major < 0 then
		print("Warning: _RUNTIME.version.semantic.major is negative")
	end
	if _RUNTIME.version.semantic.minor < 0 then
		print("Warning: _RUNTIME.version.semantic.minor is negative")
	end
	if _RUNTIME.version.semantic.patch < 0 then
		print("Warning: _RUNTIME.version.semantic.patch is negative")
	end

	if _RUNTIME.version.semantic.prerelease ~= nil and type(_RUNTIME.version.semantic.prerelease) ~= "string" then
		error("when present, _RUNTIME.version.semantic.prerelease must be a string")
	end
	if _RUNTIME.version.semantic.build ~= nil and type(_RUNTIME.version.semantic.build) ~= "string" then
		error("when present, _RUNTIME.version.semantic.build must be a string")
	end
end
if _RUNTIME.version.git ~= nil then
	if type(_RUNTIME.version.git) ~= "table" then
		error("when present, _RUNTIME.version.git must be a table")
	end
	if not table.isfrozen(_RUNTIME.version.git) then
		error("when present, _RUNTIME.version.git must be frozen")
	end
	if _RUNTIME.version.git.url ~= nil then
		if type(_RUNTIME.version.git.url) ~= "string" then
			error("when present, _RUNTIME.version.git.url must be a string")
		end
		if not isProbablyGitUrl(_RUNTIME.version.git.url) then
			print("Warning: _RUNTIME.version.git.url doesn't look like a web or git URL")
		end
	end
	if type(_RUNTIME.version.git.commit) ~= "string" then
		error("when .git present, _RUNTIME.version.git.commit must be present and a string")
	end
	if _RUNTIME.version.git.branch ~= nil and type(_RUNTIME.version.git.branch) ~= "string" then
		error("when present, _RUNTIME.version.git.branch must be a string")
	end
end
if _RUNTIME.url ~= nil then
	if type(_RUNTIME.url) ~= "string" then
		error("when present, _RUNTIME.url must be a string")
	end
	if not isProbablyGitUrl(_RUNTIME.url) then
		print("Warning: _RUNTIME.url doesn't look like a URL")
	end
end
if _RUNTIME.extra ~= nil then
	if type(_RUNTIME.extra) ~= "table" then
		error("when present, _RUNTIME.extra must be a table")
	end
	if not table.isfrozen(_RUNTIME.extra) then
		error("when present, _RUNTIME.extra must be frozen")
	end
end

-- Validation for _LUAU
if _LUAU then
	if type(_LUAU) ~= "table" then
		error("_LUAU must be present and a table")
	end
	if not table.isfrozen(_LUAU) then
		error("_LUAU must be frozen")
	end

	if _LUAU.url ~= nil then
		if type(_LUAU.url) ~= "string" then
			error("when present, _LUAU.url must be a string")
		end
		if not isProbablyGitUrl(_LUAU.url) then
			print("Warning: _LUAU.url doesn't look like a web or git URL")
		end
	end

	if type(_LUAU.version) ~= "table" then
		error("_LUAU.version must be present and a table")
	end
	if not table.isfrozen(_LUAU.version) then
		error("_LUAU.version must be frozen")
	end
	if type(_LUAU.version.display) ~= "string" then
		error("_LUAU.version.display must be present and a string")
	end
	if type(_LUAU.version.major) ~= "number" then
		error("_LUAU.version.major must be present and a number")
	end
	if type(_LUAU.version.minor) ~= "number" then
		error("_LUAU.version.minor must be present and a number")
	end

	if _LUAU.version.major ~= 0 then
		print("Warning: Luau major version is not 0. This is unusual!")
	end
	if _LUAU.version.display ~= `{_LUAU.version.major}.{_LUAU.version.minor}` then
		print("Warning: Luau display version does not match declared major and minor components. This is unusual!")
	end
else
	print("_LUAU not defined (allowed), skipping validation")
end

print("All validation passed! Current values:")
print("_RUNTIME:", _RUNTIME)
print("_LUAU:   ", _LUAU)
