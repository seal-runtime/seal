-- This script is run by seal's git pre-commit hook, before precommit.luau.
-- Cargo.toml's [patch] block points mluau at a local checkout for Luwu work. It stays commented out in commits so
-- everyone else can build seal, so a commit with it enabled is refused.
--
-- Checks the staged Cargo.toml, or the file given as the first argument:
--   seal src/scripts/check_local_patch.luau Cargo.toml

const env = require("@std/env")
const fs = require("@std/fs")
const process = require("@std/process")
const str = require("@std/str")
const output = require("@std/io/output")

const path = env.args[1]

const function read_cargo_toml(): string?
    if path then
        return fs.readfile(path)
    end

    const staged = process.run({ program = "git", args = { "show", ":Cargo.toml" } })
    return if staged.ok then staged.stdout else nil
end

const contents = read_cargo_toml()
if not contents then
    -- no Cargo.toml in the index: nothing to check
    process.exit(0)
end

const function mentions(line: string, word: string): boolean
    return string.find(line, word, 1, true) ~= nil
end

for _, line in str.splitlines(contents) do
    const trimmed = str.trimfront(line, " ")
    const local_dependency = (str.startswith(trimmed, "mluau") or str.startswith(trimmed, "mlua-sys")) and mentions(trimmed, "path")

    if str.startswith(trimmed, "[patch.") or local_dependency then
        output.ewriteln("you forgot to comment out mluau local patch u goober")
        output.ewriteln(`  {path or "Cargo.toml (staged)"}: {line}`)
        process.exit(1)
    end
end
