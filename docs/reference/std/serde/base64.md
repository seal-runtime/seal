<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# serde.base64

`local base64 = require("@std/serde/base64")`

$hspace{5pt}$good for serving binary stuff in a digestable form for serving things on the internet

base64.encode: `(data: string | buffer) -> string`

base64.decode: `(data: string) -> buffer`

base64.urlsafe.encode: `(data: string | buffer) -> string`

base64.urlsafe.decode: `(data: string) -> buffer`
