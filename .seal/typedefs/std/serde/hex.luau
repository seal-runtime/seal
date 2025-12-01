--[=[
Deal with hex data.
]=]
export type hex = {
    encode: (data: buffer | string) -> string,
    decode: (encoded: string) -> buffer,
}

return {} :: hex