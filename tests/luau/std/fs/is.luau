local fs = require("@std/fs")
local env = require("@std/env")
local str = require("@std/str")
local time = require("@std/time")
local process = require("@std/process")

local function is()
    local data_path = fs.path.canonicalize(fs.path.join(".", "tests", "data"))

    if env.os == "Linux" or env.os == "MacOS" then
        local symlink_path = fs.path.join(data_path, "cats_link.json")
        fs.symlink(fs.path.join(data_path, "cats.json"), symlink_path)
        local MAKE_SOCKET_SRC = str.unindent([[
            import socket
            import os
    
            socket_path = "./tests/data/socky.sock"
    
            # Ensure parent directory exists
            os.makedirs(os.path.dirname(socket_path), exist_ok=True)
    
            # Remove existing socket if present
            try:
                os.unlink(socket_path)
            except FileNotFoundError:
                pass
    
            sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            sock.bind(socket_path)
            sock.listen(1)
        ]], "Spaces", 8)
    
        fs.file.try_remove(fs.path.join(data_path, "socky.sock"))
    
        local child = process.spawn {
            program = "python3",
            args = { "-" }, -- read from stdin
        } :: process.PipedChild

        child.stdin:write(MAKE_SOCKET_SRC)
        child.stdin:close()
        while child:alive() do
            time.wait(0.1)
        end
    end

    for _, filepath in fs.listdir(data_path) do
        local is = fs.is(filepath)
        local basename = fs.path.child(filepath)
        assert(basename ~= nil, `where basename for {filepath}`)
        if basename == "csv_data" then
            assert(is == "Directory", "csv_data should be a directory")
            print("found csv_data")
        elseif basename == "cats.json" then
            assert(is == "File", "cats.json should be a file")
            print("found cats.json")
        elseif basename == "socky.sock" then
            assert(is == "UnixSocket", "socky.sock should be a UnixSocket")
            print("found socky.sock")
        elseif basename == "cats_link.json" then
            print(`found symlink cats_link.json that points to {fs.readlink(filepath)}`)
            assert(is == "Symlink", "cats_link.json should be a Symlink")
        end
    end
    
    fs.file.try_remove(fs.path.join(data_path, "socky.sock"))
    fs.file.try_remove(fs.path.join(data_path, "cats_link.json"))
end

is()