local fs = require("@std/fs")
local input = require("@std/io/input")
local prompt = require("@std/io/prompt")

local function pickanimals()
    local animals: { string } = {}
    print("Animals to pick from? (ctrlc when done to stop)")
    local index = 1
    while true do
        local response = input.readline(`animal {index}: `)
        if typeof(response) == "interrupt" then
            break
        elseif typeof(response) == "string" then
            table.insert(animals, response)
            index += 1
        end
    end
    
    local default = tonumber(prompt.text("default (index)?"))
    
    local result = prompt.pick("Favorite animal?", animals, default)
    
    print(`you picked {animals[result]}`)
end

pickanimals()

-- checks if we fixed the issue where running prompt.confirm after prompt.pick
-- causes prompt.pick not to remove all its previous output
local function confirmshadowspick()
    print("hi")
    print("bye")
    
    local index = prompt.pick("which animal?", { "cats", "dogs", "seals", "ostriches", "elephants", "badgers", "sloths" }, 1)
    local i2 = prompt.pick("second", { "first", "second", }, 2)
    if prompt.confirm("some action") then
        print("confirmed")
    end
end

confirmshadowspick()

local function manyoptions()
    local files = fs.listdir(".")
    print(`we have {#files}`)
    local file = prompt.pick("pick a file in your cwd", files, 1)
end

manyoptions()