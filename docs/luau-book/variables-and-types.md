# Variables and (Basic) Types

To define a variable (binding) in Luau, use the `local` keyword.

```luau
local cats = 1
```

Variable names (more correctly called identifiers) must start with an alphabetical character (a-z/A-Z) or underscore (_), and may contain letters, numbers, and underscores.

By convention, local variables should be `camelCase` or `snake_case`, types and oop-like classes should be `PascalCase`, and constants should be `SCREAMING_SNAKE_CASE`. Naming conventions vary across Luau environments, but *seal* generally sticks to `luaucase` for function names and `snake_case` for properties, methods, and local variables.

The main primitive data types in Luau are `number`, `string`, `boolean`, `nil`,  `function`, `buffer`, `vector`, and `table`. Structs reflected from C or Rust are called `userdata` or `extern type`.

To prevent a binding from changing type later in the program, use an explicit type annotation:

```luau
local cats: number = 2
cats = "meow" -- TypeError: Type 'string' could not be converted into 'number'
```

If you don't see a TypeError appear here, ensure strict mode is enabled in your `.luaurc` or `.config.luau` file. Luau works best in strict mode. You can additionally enable strict mode on a file-by-file basis by adding `--!strict` to the top of your Luau file.

To signify that a value is *optional* (can be `nil`), put a `?` after the annotated type:

```luau
local result: string?
for key, value in map do
    if key == "hina" then
        result = value
        break
    end
end
if result then
    print(`found result {result}!`)
end
```

In practice, most `local` variables need not be explicitly annotated.

## Strings

Strings in Luau are immutable (cannot be directly modified) and can contain arbitrary bytes. Unlike Rust, Luau strings don't have to be valid UTF-8!

Use the following string literal syntaxes to define strings:

```luau
local double_quoted = "i am a stringy string"
local single_quoted = 'i can contain "double quotes"!'
local long_raw_string = [[
    i can be one or more lines and literally contain \n and \z without needing escapes!
]]
local interpolated = `i can contain single ' and double " quotes and contain any {
    if math.random(1, 10) < 5 then
        "arbitrary "
    else
        ""
}expression!`
```

To concatenate (combine) strings, use the dedicated concat `..` operator:

```luau
local combined = double_quoted .. interpolated
```

Use `string` library functions to access parts of strings or search strings with pattern matching:

```luau
-- note: you cannot index strings directly, you have to use string.sub
local first_three_bytes = string.sub("one two three", 1, 3)
print(first_three_bytes) --> "one"
-- luau does not have regex, but we have string patterns!
local first_part = string.match("grab the first word", "^(%w+)%s)")
print(first_part) --> "grab"
```

To build strings in a loop, use `table.concat` on an array of strings:

```luau
local result: { string } = {}
for key, value in map do
    table.insert(result, `{key} = {value}`)
end
print(table.concat(result, "\n"))
```

## Numbers

All numbers in Luau are floating point numbers (f64) accurately representable up to 2^52.

```luau
local three = 3
-- number literals can contain underscores for readability
local hundred_thousand = 100_000
local binary_number = 0b1010
local hexxy = 0x12
```

## Functions

To define a function, use the following syntax:

```luau
-- a local function:
local function increment(n: number, amount: number): number
    return n + amount
end
-- the exact same function, except using anonymous function syntax:
local increment = function(n: number, amount: number): number
    return n + amount
end
```

By convention, most function parameters and return types should be annotated.

If a function returns multiple values, surround its return types with parentheses.

```luau
local function minmax(a: number, b: number): (number, number)
    if a < b then
        return a, b
    else
        return b, a
    end
end
```

Multiple returns (multirets) are often incorrectly referred to as tuples, which can be confusing to people who are used to tuples in other languages that act as primitive value types.

To call a function, use parentheses:

```luau
local incremented = increment(1, 2) 
print(incremented) --> 3
local min, max = minmax(2, 1)
```

Unlike many other languages, you can *omit* parentheses if you call a function with only a single argument -- if the argument is a string literal or a table literal:

```luau
local result: { string } = {}
local function push(s: string)
    table.insert(result, s)
end
push "hi"
push "bye"

local process = require("@std/process")
local result = process.run {
    program = "seal",
    args = { "run" },
}
```

Omitting parentheses is considered bad practice for string calls and usually discouraged for table calls except in the context of DSLs. *seal* however considers table calls acceptable.

## Tables: structs, arrays, maps, and classes

The base data structure in Luau is the `table`. Tables can be used like structs, arrays, and maps (dictionaries), or can be composed in more complex ways to represent any data structure.

At runtime, all tables consist of key-value pairs. Keys can be any hashable value (including other tables, `extern` types, vectors, etc.) except `NaN` (not a number) and `nil`. Values can be any value type except `nil`. To remove an element from a table, set its value to `nil`.

For ease of communication and typechecking, we broadly classify tables into the categories of array-like, map-like, struct-like, set-like, and oop-like (implementations and objects).

### Arrays

An array-like table can be defined like this:

```luau
local values = {1, 2, 3, 4}
local strings = { "hi", "bye", "seals" }
```

To append elements to an array-like table, use `table.insert` with 2 arguments:

```luau
table.insert(strings, "meow")
print(strings) --> { [1] = "hi", [2] = "bye", [3] = "seals", [4] = "meow" }
```

To find the index of an element in an array-like table, use `table.find`.

```luau
local index = table.find(strings, "bye")
```

Be sure to never pass the result of `table.find` directly into a `table.remove` call without `nil`-checking its return:

```luau
-- BAD! footgun! removes last element of strings if table.find returns nil
table.remove(strings, table.find(strings, "bye"))
-- do this instead:
local index = table.find(strings, "bye")
if index then
    table.remove(strings, index)
end
``

An array-like table can contain values of different types:

```luau
local values: { number | string } = { "hi", 1, "bye", 2 }
```

To get the length of an array-like table, use the `#` length operator:

```luau
print(#strings) --> 3 once "bye" is removed
```

The first element in an array-like table is at index `1`, not `0` like in languages like Rust or Python.

Array-like tables are contiguous maps of `numbers` to `strings`. We say *contiguous* because if an array has holes in it (like `{ "hi", "bye", nil, "uhh" }`) it cannot be iterated-through properly, causing a bunch of problems.

In typechecking, Luau treats a list of strings (`{ string }`) identically to a map of numbers to strings (`{ [number]: string }`) although users often use the latter to indicate holes are expected.

A map-like table can be defined like this:

```luau
local elements: { [string]: string? } = {
    first = "hi",
    last = "bye",
    seals = "seals",
    ["key with a space"] = "is allowed",
}
```

To index a map-like table, use `t[key]` syntax, or `t.key` syntax if the key is a string that could be a valid identifier (doesn't contain spaces and doesn't include any symbols other than `_`).

If the key doesn't exist in the table, the index operation results in `nil` instead of erroring. This is why it's good practice to annotate the value type of a map-like table with `?`, to remind you to `nil`-check the result.

```luau
-- BAD: passes typechecking but errors at runtime
local checked_elements: { [string]: string } = {
    hi = "hehe",
}
print("user says " .. checked_elements.hii)

-- better:
local elements: { [string]: string? } = {
    hi = "hehe"
}
local hi = elements.hi
if hi then
    print("user says " .. hi)
end
```

A struct-like table will always have the same keys (properties), and more properties cannot later be added to one without causing a TypeError.

```luau
local cat: {
    name: string,
    age: number
} = {
    name = "Taz",
    age = 12,
}
```

To use struct-like tables more effectively, use a `type` alias binding:

```luau
type Cat = {
    name: string,
    age: number,
}

local cat: Cat = {
    name = "Taz", -- Luau would throw a TypeError if any of these fields were incompatible
    age = 12,
}
```

Luau has special syntax for adding functions to tables:

```luau
local Cat = {}
function Cat.new(name: string, age: number)
    return {
        name = name,
        age = age,
    }
end
```

## Buffers

Buffers are contiguous, fixed-size slices in memory that can be mutated in place.

Buffer offsets start at `0` since they represent offsets in memory, unlike tables indices which start at `1`.

To create a buffer, use `buffer.create(size)`. Buffers can be modified using `buffer` library functions.

## Vectors

Vectors are immutable containers of three numbers, heavily optimized for mathematical vector library operations, that can be used to represent 2D and 3D coordinates as well as colors.

To create a vector, use `vector.create(x, y, z)`. If `z` isn't specified, it will default to `0`. Vectors can be added, subtracted, multiplied, divided, and floor divided.

```luau
local position1 = vector.create(1, 0, 1)
local position2 = vector.create(122, -12, 47)

local distance = vector.magnitude(position2 - position1)
print(distance) --> 130.00384521484375
```
