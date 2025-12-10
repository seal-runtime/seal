local s, r = pcall(function()
    return (require :: any)("./meow!/./*")
end)

if typeof(r) == "error" then
    if tostring(r):match("PANIC") then
        error(`we panicked but managed to catch the panic (crazy): {r}`)
    else
        -- we good
    end
else
    error(`wtf this was supposed to error but actually got: {r}`)
end
