local http = require("@std/net/http")

if channel and channel.data then
	local content = channel.data :: { url: string }
	local result = http.get {
		url = content.url
	}
	if result.ok then
		if result.body:match("^{") then
			channel:send(result:expect_json())
		else
			channel:send(result.body)
		end
	end
elseif not channel then
	error("run me in a thread.spawn!!")
end
