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