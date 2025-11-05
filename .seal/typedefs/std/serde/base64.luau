--[=[
good for serving binary stuff in a digestable form for serving things on the internet
]=]
return _LOAD_EXTERN("@seal/std/serde/base64") :: typeof({
	encode = function(data: string | buffer): string
		return nil :: any
	end,
	decode = function(data: string): buffer
		return nil :: any 
	end,
	urlsafe = {
		encode = function(data: string | buffer): string
			return nil :: any
		end,
		decode = function(data: string): buffer
			return nil :: any
		end,
	}
})