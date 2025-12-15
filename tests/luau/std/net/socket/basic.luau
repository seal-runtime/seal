local format = require("@std/io/format")
local socket = require("@std/net/socket")

local websocket = socket.connect("wss://ws-feed.exchange.coinbase.com")
assert(typeof(websocket) == "Websocket", `got typeof Websocket: {typeof(websocket)}`)

assert(websocket:sendable() == true, "we should be able to send messages through the websocket")

websocket:send({
    type = "subscribe",
    channels = {
        { name = "ticker", product_ids = { "BTC-USD" } }
    },
})

type CoinbaseResponse = {
    best_ask_size: string,
    best_ask: string,
    best_bid: string,
    best_bid_size: string,
    high_24h: string,
    last_size: string,
    low_24h: string,
    open_24h: string,
    product_id: string,
    price: string,
    sequence: number,
    side: string,
    type: string,
    time: string,
    trade_id: number,
    volume_30d: string,
    volume_24h: string,
}

local first = true
local start_time = os.clock()

while (os.clock() - start_time) < 6 do
    local message = websocket:read()
    if not message:is_utf8() then
        warn(`message not utf8, got: {format.hexdump(message:as_bytes())}`)
        continue
    end
    
    local response = message:try_json<<CoinbaseResponse>>()
    if typeof(response) == "error" then
        warn(`unable to decode json for message {message:as_string()} due to err: {tostring(response)}`)
        continue
    end

    if first then
        assert(response.type == "subscriptions", `first response should be subscriptions, got {message.type}`)
        first = false
    else
        assert(response.type == "ticker", "type should be ticker from api")
    end
end
