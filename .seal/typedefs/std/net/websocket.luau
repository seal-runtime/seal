--[=[
    By default, the websocket library trusts only system certs 
    and will fallback to Mozilla's CAs only if no certs are found.

    To provide custom SSL certs, set `SSL_CERT_FILE` to a `.pem` certificate file,
    and to disable system certs set `SEAL_SYSTEM_CERTS=false`.

    - If `SEAL_SYSTEM_CERTS=false` and an `SSL_CERT_FILE` is not provided, falls back to
    Mozilla's public CA. 
    - If an `SSL_CERT_FILE` is provided and `SEAL_SYSTEM_CERTS=false`, seal trusts only the `SSL_CERT_FILE`.
]=]
export type socket = {
    --[=[
        Connect to the WebSocket at `url` as a client, returning a `Websocket` handle.

        To send messages on the WebSocket, use `Websocket:send`; to receive messages, use `Websocket:read`.
    ]=]
    connect: (url: string) -> Websocket,
}

export type CloseCode = 
    | "Normal"
    | "Away"
    | "Protocol"
    | "Unsupported"
    | "Status"
    | "Abnormal"
    | "Invalid"
    | "Policy"
    | "Size"
    | "Extension"
    | "Error"
    | "Tls"
    | number

return {} :: socket