--[=[
    By default, the http and websocket libaries trust system certs, and have
    differing behavior if zero system certs are found.
    - The http library will throw errors due to ureq internals not allowing me
    to write a happier fallback path, to handle this
    - The websocket library will fall back to Mozilla CAs.

    To provide custom SSL certs, set `SSL_CERT_FILE` to a `.pem` certificate file,
    and to disable system certs set `SEAL_SYSTEM_CERTS=false`.

    - If `SEAL_SYSTEM_CERTS=false` and an `SSL_CERT_FILE` is not provided, falls back to
    Mozilla's public CA. 
    - If an `SSL_CERT_FILE` is provided and `SEAL_SYSTEM_CERTS=false`, seal trusts only the `SSL_CERT_FILE`.
]=]
local net = {}

net.http = require("@std/net/http")
net.websocket = require("@std/net/websocket")

return net