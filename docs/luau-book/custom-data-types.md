# Custom Data Types

Luau allows you to get expressive with your datatypes. You can make anything you want using tables, including custom containers, oop-like objects, stacks, queues, lists, or whatever else you need. Unlike many other languages, Luau programmers often don't use abstract data types as often, instead preferring to use plain tables with predefined shapes as necessary.

For example, to emulate a stack, you could just use `table.insert` and `table.remove` on an array-like table:

```luau
local stacky: { string } = {}

local function push(s: string)
    table.insert(stacky, s)
end
local function pop(): string?
    return table.remove(stacky, 1)
end
local function peek(): string?
    return stacky[1]
end
```

In reality, Luau data types are often just a type definition that represents business logic. For example:

```luau
--- Represents a currently-running test file
type TestCase = {
    --- Name of the test case
    name: string,
    --- Whether we're ignoring this test
    ignored: boolean,
    --- The test running function, must return `true` if the test passed,
    --- false if the test fails, or an error object if the test function errored out.
    fn: () -> err.Result<boolean>,
}
type TestFile = {
    path: string,
    cases: { TestCase },
}

local test_files: { TestFile } = {}
for _, filepath in fs.filter("./tests", "*.luau", "-*.ignore.luau") do
    local result = luau.eval(fs.readfile(filepath), {
        name = filepath,
        stdlib = "seal",
    })
    assert(
        typeof(result) ~= "error", 
        `file {filepath} unexpectedly errored: {result}`
    )
    table.insert(test_files, {
        path = filepath,
        cases = result :: { TestCase },
    })
end
```

If you're familiar with object oriented programming, you might want to reach for classes. If you're familiar with structs and implementations in Rust, Luau's answer might seem familiar to you.

To make a class in Luau, you need a table that represents the class' implementation and a constructor function that makes instances of that class.

To prevent allocating new functions (for each method) every time you call the class' constructor, we use metatables to redirect the `__index` operation from a class instance's struct to its implementation. Luau specifically optimizes methodcalls when a metatable's `__index` points to itself.

```luau
local User = {}
User.__index = User

type UserProps = {
    name: string,
    birthday: DateTime,
    posts: { string },
}
export type User = typeof(User.new(...))

function User.new(name: string, dob: string)
    local self: UserProps = {
        name = name,
        birthday = datetime.parse(dob, "MM/DD/YYYY"),
        posts = {},
    }
    return setmetatable(self, User)
end

function User.post(self: User, title: string, content: string): Post
    local new_post = Post.new(title, content)
    table.insert(self.posts, new_post)
    return new_post
end
```

To make a new instance of `User`, call `User.new` like any function. To call a method on it, use methodcall syntax `:`

```luau
local taz = User.new("Taz", "06/10/2019")
local new_post = taz:post("Meow", "Today I made a meow noise that sounded like a bird")
```
