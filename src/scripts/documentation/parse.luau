--!optimize 2
local parser = require("@libraries/LuauParser/Parser")
local fs = require("@std/fs")
local format = require("@std/io/format")
local str = require("@std/str")
local time = require("@std/time")

const syntax = require("@libraries/LuauParser/Parser/Syntax")

const components = require("./components")
const Documentation = components.Documentation
type Documentation = components.Documentation
type HeadingComponent = components.HeadingComponent
type TextComponent = components.TextComponent
type CodeblockComponent = components.CodeblockComponent
type ListComponent = components.ListComponent

const comment = require("./comments")
const BlockComment = comment.BlockComment
const LineComment = comment.LineComment
type BlockComment = comment.BlockComment
type LineComment = comment.LineComment

local path = "/home/deviaze/Repositories/seal/.seal/typedefs/std/fs/init.luau"
path = string.gsub(path, fs.path.cwd(), ".")

const content = fs.readfile(path)
const lines = string.split(content, "\n")

format.defaults {
    indent_spaces = 2
}

local parsed = parser.parse(content, {
    captureComments = true,
})

assert(typeof(parsed) ~= "error", "not an error")


const bytes_by_line: { number } = {}
do
    local bytes = 0
    for index, line in lines do
        bytes += #line + 1 -- include length of stripped \n
        bytes_by_line[index] = bytes
    end
end

class Library
    public sublibraries: { Library }
    public types
end

type Property = 
    | Function
    | Field
    | TableType
    | Array
    | Map
    | StringEnum
    | ComplexEnum

class TableType
    public name: string
    public docs: Documentation
    public properties: { Property }
end
class Param
    public name
    public type
    function __tostring(self)
        if self.name ~= "" then
            return `{self.name}: {self.type}`
        else
            return self.type
        end
    end
end
class Function
    public name: string
    public location: vector
    public parameters: { Param }
    public returns: { string }
    public docs: Documentation?

    function from_ast(name: string, node: syntax.AstTypeFunction)
        const params: { Param } = {}
        for i, arg in node.argNames do
            const param_type = node.argTypes.types[i]
            if param_type and param_type.kind == "TypeReference" then
                table.insert(params, Param {
                    name = if typeof(arg) ~= "boolean" then arg.name else "",
                    type = param_type.name
                })
            else
                print(param_type)
            end
        end
        const rets = {}
        if node.returnTypes.kind == "TypePackExplicit" then
            for _, return_type in node.returnTypes.types do
                if return_type.kind == "TypeReference" then
                    table.insert(rets, return_type.name)
                end
            end
        end
        -- node.location.begin.
        return Function {
            name = name,
            parameters = params,
            returns = rets,
            location = vector.create(node.location.begin.column, node.location.begin.line)
        }
    end
    function __tostring(self)
        const params = {}
        for _, p in self.parameters do
            table.insert(params, tostring(p))
        end
        if #self.returns >= 1 then
            return `function {self.name}({table.concat(params, ", ")}) -> {table.concat(self.returns)} [line: {self.location.y}:{self.location.x}]`
        else
            return `function {self.name}({table.concat(params, ", ")}) -> () [line: {self.location.y}:{self.location.x}]`
        end
    end
end
class Field
    public docs: Documentation
    public value: Property
end
class Array
    public docs: Documentation?
    public inner: string
end
class Map
    public docs: Documentation?
    public key_indexer: string
    public value_result: string
end
class StringEnum
    public docs: Documentation
    public variants: {
        {
            name: string,
            docs: Documentation?,
        }
    }
end
class ComplexEnum
    public docs: Documentation
    public variants: { Property }
end

const docs: { Documentation } = {}
const comments: { BlockComment | LineComment } = comment.build(parsed, lines, path, content)
for index, comm in comments do
    local text: string, last: number
    if class.isinstance(comm, BlockComment) then -- LUAU FIXME: refines to never
        comm = comm :: BlockComment
        text, last = comm:extract_from_lines(lines, bytes_by_line, content)
        if str.startswith(text, "--[[") then -- not --[=[]=]
            continue
        end
        table.insert(docs, Documentation.from(text, last))
    elseif class.isinstance(comm, LineComment) then -- LUAU FIXME: refines to never
        comm = comm :: LineComment
        text, last = comm:extract_from_lines(lines, bytes_by_line, content)
        if str.startswith(text, "---") then
            text = str.trimfront(text, "%-%-%-")
            table.insert(docs, Documentation.from(text, last))
        end
    end
end

const function stdlib_require_call(v: syntax.AstStatLocal): string?
    const first = v.values[1]
    if first.kind ~= "ExprCall" then
        return nil
    end

    const func = first.func
    if func.kind ~= "ExprGlobal" then
        return nil
    end

    if func.name == "require" then
        const arg = first.args[1]
        if arg.kind == "ExprConstantString" and str.startswith(arg.value, "@") then
            return arg.value
        end
    end

    return nil
end
assert(parsed.root)
const stdlibs: { [string]: string } = {}
format.defaults {
    indent_spaces = 2,
    max_depth = 2
}

const things = {}
for i, v in parsed.root.body do
    if v.kind == "StatLocal" and #v.values > 0 and #v.vars > 0 then
        const stdlib_path = stdlib_require_call(v)
        if stdlib_path then
            const binding = v.vars[1].name
            stdlibs[binding] = stdlib_path
        end
    elseif v.kind == "StatTypeAlias" then
        if v.type.kind == "TypeTable" then
            -- print(v.type.props[1])
            for _, prop in v.type.props do
                if prop.type.kind == "TypeFunction" then
                    const f = Function.from_ast(prop.name.value, prop.type)
                    table.insert(things, f)
                end
            end
        end
    else
        print(v)
        break
    end
end
print(things)