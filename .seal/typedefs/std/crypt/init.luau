local crypt = {}

crypt.aes = require("@self/aes")
crypt.rsa = require("@self/rsa")
crypt.hash = require("@self/hash")
crypt.password = require("@self/password")

return crypt