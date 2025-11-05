export type Hex = {
	encode: (data: buffer | string) -> string,
	decode: (encoded: string) -> buffer,
}

return _LOAD_EXTERN("@seal/std/serde/hex") :: Hex