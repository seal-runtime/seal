local fs = require("@std/fs")
local base85 = require("@std/serde/base85")

local nanuk_buffy = fs.readbytes("./tests/data/nanuk.png")
local sensible_nanuk = base85.encode(nanuk_buffy)

local function serdeworks()
	local nanuk_encoded_85 = base85.encode(fs.readbytes("./tests/data/nanuk.png"))
	local nanuk_decoded_85 = base85.decode(nanuk_encoded_85)
	assert(buffer.tostring(nanuk_buffy) == buffer.tostring(nanuk_decoded_85), "nanuks don't match")
end

serdeworks()
