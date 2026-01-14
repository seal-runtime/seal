--[=[
good for serving binary stuff in a digestable form for serving things on the internet

almost exactly the same as `base64`, with a slightly smaller footprint in the form
of a ~25% expected binary size increase when encoded vs. a ~33% increase from base64
]=]
export type base85 = {
	encode: (data: string | buffer) -> string,
	decode: (data: string) -> buffer,
}

return {} :: base85
