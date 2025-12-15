export type socket = {
    connect: (url: string) -> Websocket,
}
return {} :: socket