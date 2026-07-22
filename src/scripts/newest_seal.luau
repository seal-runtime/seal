-- finds whichever seal build is newest for precommit hook

const fs = require("@std/fs")
const env = require("@std/env")
const datetime = require("@std/time/datetime")

const base_seal = env.executable_path
const seal_options = {
    "./target/release/seal",
    "./target/debug/seal"
}

const function get_modified_time(path: string): DateTime?
    const entry = fs.file(path)
    if not entry then
        return nil
    end
    const modified = entry:metadata().modified_at
    return modified
end

local newest = {
    path = base_seal, -- we know system seal exists at this path
    modified = get_modified_time(base_seal) or (datetime.now() - datetime.days(14))
}

for _, path in seal_options do
    const modified_time = get_modified_time(path)
    if modified_time and modified_time > newest.modified then
        newest = {
            path = path,
            modified = modified_time
        }
    end
end

print(newest.path)
