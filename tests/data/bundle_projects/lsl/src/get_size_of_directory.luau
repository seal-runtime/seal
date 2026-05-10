local fs = require("@std/fs")

--- recursively gets size of `dir` in bytes
--- returns (partial size, error) if any subdirectories were inaccessible, otherwise (full size, nil)
local function get_size_of_directory(dir: string): (number, string?)
    local ok, paths = pcall(fs.listdir, dir)
    if not ok then
        return 0, dir
    end

    local size = 0
    local failed: { string } = {}
    for _, path in paths do
        -- using fs.is instead of just fs.file bc paths like wine's dosdevices symlinks
        -- (z:/dev/...) cause fs.file to throw on invalid paths
        local path_is = fs.is(path)
        if path_is == "File" then
            local entry = fs.file(path)
            if entry then
                size += entry:size()
            end
        elseif path_is == "Directory" then
            local sub_size, sub_err = get_size_of_directory(path)
            size += sub_size
            if sub_err then
                table.insert(failed, sub_err)
            end
        end
        -- skip symlinks, sockets, fifos, devices, etc.
    end

    if #failed > 0 then
        return size, table.concat(failed, "\n")
    end

    return size, nil
end

return get_size_of_directory
