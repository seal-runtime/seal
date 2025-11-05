--[=[
good for serving binary stuff in a digestable form for serving things on the internet
]=]
export type base64 = {
    encode: (data: string | buffer) -> string,
    decode: (data: string) -> buffer,
    urlsafe: {
        encode: (data: string | buffer) -> string,
        decode: (data: string) -> buffer
    }
}

return {} :: base64