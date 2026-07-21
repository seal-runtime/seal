<img align="right" width="220" height="220" alt="LuauParser" src="https://github.com/user-attachments/assets/69223bf1-05a5-4825-baa6-c9603cb768bf" />

<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h1> LuauParser </h1>
    </summary>
  </ul>
</div>

![Ver](https://img.shields.io/badge/version-v0.715-blue?style=plastic)
[![License](https://img.shields.io/badge/license-MIT-97ca00?style=plastic)](https://github.com/vantoanvh/LuauParser/blob/main/LICENSE)
[![Docs](https://img.shields.io/badge/documentation-online-red?style=plastic&logo=github)](https://vantoanvh.github.io/LuauParser)
[![Release](https://img.shields.io/badge/release-latest-darkblue?style=plastic&logo=luau&logoColor=white)](https://github.com/vantoanvh/LuauParser/releases/latest)
[![Release](https://img.shields.io/badge/devforum-resource-blue?style=plastic&logo=lua&logoColor=white)](https://devforum.roblox.com/t/luauparser-full-v0710/4482837)

A modern, fully featured **[Lua*u*](https://luau.org/)** parser written entirely in Luau<br>
Fully strict-typing with the New Type Solver and optimized for high performance.

## Introduction
> Only compatibles with <strong>New Type Solver</strong>.

A complete port of **[Parser.cpp](https://github.com/luau-lang/luau/blob/master/Ast/src/Parser.cpp)** providing both *AST* **(Abstract Syntax Tree)** & *CST* **(Concrete Syntax Tree)**.<br>
**To stay true to the original implementation**, the CST does not include low-level trivia such as whitespace.

This Lua*u* parser is primarily intended for plugin development and for building Lua*u* compilers or tooling.<br>
It provides detailed syntax through both **AST** and **CST**, making it well suited for advanced language tooling such as *linters, formatters, highlighter, refactoring tools,* or even a full *luau compiler/transpiler.*

## Performance

This parser is fast because it uses a simple singleton-style procedural design. It just resets its internal variables and runs, avoiding the overhead of more complex architectures.

## Code Examples
```luau
local Parser = require(path.to.LuauParser)

local success, result = Parser.parse('const foo: (string, ...number) -> (...any) = 123i')

if success then
	print(result.root) -- do anything you want
else
	warn(result.errors)
end
```
