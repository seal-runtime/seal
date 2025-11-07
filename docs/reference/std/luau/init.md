<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# luau

`local luau = require("@std/luau")`

luau.eval: `(src: string, options: EvalOptions?) -> unknown | error`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Evaluate Luau source code from a string in the current Luau VM.
$\hspace{5pt}$
$\hspace{5pt}$ By default, this function evaluates in `"safe"` mode with only Luau's standard library (minus some deprecated environment breaking functions).
$\hspace{5pt}$
$\hspace{5pt}$ ### `EvalOptions` options:
$\hspace{5pt}$
$\hspace{5pt}$ `name` represents the `chunk_name` of the evaluated src.
$\hspace{5pt}$
$\hspace{5pt}$ `stdlib` can be one of the following (or left unspecified, in which it defaults to `"safe"`):
$\hspace{5pt}$
$\hspace{5pt}$ - `"safe"` - The evaled code will have access to most libraries/functions that come with Luau,
$\hspace{5pt}$ but nothing that can access your file system or the internet.
$\hspace{5pt}$ - `"seal"` - The evaled code will have access to anything seal can do, from accessing environment variables to creating an infinite number of empty files in your home directory.
$\hspace{5pt}$ - `"none"` - Disable every single global Luau comes with, including `tostring` and `print`.
$\hspace{5pt}$
$\hspace{5pt}$ ## Returns
$\hspace{5pt}$
$\hspace{5pt}$ Either whatever the source code evaluates to (`unknown`), or a tostringable userdata instance representing
$\hspace{5pt}$ an error that occurred when evaluating the code, such as a syntax error or runtime error.
$\hspace{5pt}$
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if the code cannot be evaluated, but not if it contains a syntax error or errors at runtime.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$

```luau
local luau = require("@std/luau")
local src = [[return { meow = 2 }]]
local res = luau.eval(src)
local data: { meow: number } = {}
if typeof(res) == "error" then
    print(`error running code: {res}`)
else
    data.meow = (res :: any).meow
end
$\hspace{5pt}$ ```

</details>


luau.eval_unsafe: `(src: string | buffer, options: EvalOptions?) -> unknown | error`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Same as `luau.eval`, except can also accept bytecode as a string or buffer.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## ⚠️ Safety
$\hspace{5pt}$ 
$\hspace{5pt}$ This function is unsafe. You are responsible for **passing valid Luau bytecode**, and therefore
$\hspace{5pt}$ you should trust or check the bytecode you pass to this function.
$\hspace{5pt}$ 
$\hspace{5pt}$ If you pass invalid bytecode as `src`, seal will 💥 ***crash*** 💥 from an ***illegal hardware instruction***
$\hspace{5pt}$ and *coredump*.

</details>


luau.bytecode: `(src: string) -> buffer | error`

$\hspace{5pt}$ Compiles `src` to Luau bytecode.

luau.require_resolver: `() -> {`

$\hspace{5pt}$ Returns *seal*'s require resolver implementation used internally.

luau.resolve: `(requested_path: string, requiring_file_path: string) -> { err: string, path: nil } | { path: string, err: nil }`

$\hspace{5pt}$ --- Resolve a Luau require alias (`requested_path`) relative to `requiring_file_path` to find its location on the filesystem.

luau.get_aliases: `(requiring_file_path: string) -> ({ LuaurcAliases }?, string?)`

luau.expand_aliases: `(requested_path: string, aliases_by_luaurc: { LuaurcAliases }) -> (string?, string?)`

`export type` EvalOptions

EvalOptions.name: `string?`

EvalOptions.stdlib: `("seal" | "safe" | "none")?`

`export type` LuaurcAliases

LuaurcAliases.path: `string`
