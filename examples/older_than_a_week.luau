-- script that removes files older than n weeks (from provided user input)

local fs = require("@std/fs")
local err = require("@std/err")
local args = require("@std/args")
local datetime = require("@std/datetime")

-- Usage: seal ./path/to/script <path> <weeks>
local parsed = args.parse("rmbald", "removes files at <path> older than <weeks> weeks"):simple(
    args.positional("path", "path to the directory you want to remove files in (files removed recursively!!)")
        :validate(function(path: string)
            if fs.is(path) == "Directory" then
                return path
            else
                return err.message(`path not a directory: '{path}'`)
            end
        end),
    args.positional("weeks", "number of weeks old the files have to be to be removed")
        :validate(function(weeks: string)
            local n = tonumber(weeks)
            return n or err.wrap(`{n} can't be converted to number...`)
        end)
)

local folder = parsed:expect("path") :: string
local number_of_weeks = parsed:expect("weeks") :: number

local now = datetime.now()
--- a datetime that represents n weeks ago in the current timezone
local datetime_cutoff = now - datetime.days(number_of_weeks * 7)

for _, path in fs.listdir(folder, true) do -- look recursively
    -- need FileEntry to get metadata for path
    local entry = fs.find(path):try_file()
    if not entry then continue end -- might be socket or something weird

    local last_modified = entry:metadata().modified_at
    if not last_modified then
        error(`unsupported platform; can't get metadata.modified_at`)
    end

    if last_modified < datetime_cutoff then
        local weeks_old = now:since(last_modified):duration().days // 7
        print(`Removing '{path}' because it's {weeks_old} weeks old`)
        -- fs.removefile(path) -- uncomment to actually delete files
    end
end
