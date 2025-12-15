export type url = {
    --- Encodes `text` in urlsafe/percent encoding for use in urls.
    encode: (text: string) -> string,
    --- Decodes the url/percent-encoded string.
    decode: (encoded: string) -> string,
    binary: {
        encode: (data: string | buffer) -> string,
        decode: (encoded: string) -> buffer,
    }
}

return {} :: url