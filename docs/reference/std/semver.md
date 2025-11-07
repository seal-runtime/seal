<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# semver

`local semver = require("@std/semver")`

$\hspace{5pt}$ ### semver = semantic versioning
$\hspace{5pt}$
$\hspace{5pt}$ This implementation roughly follows the described spec at <https://semver.org>.
$\hspace{5pt}$
$\hspace{5pt}$ Basically, a semver consists of 3 components: major, minor, and patch, with an optional
$\hspace{5pt}$ `-rc.<number>` release candidate suffix or `+buildnumber` build suffix:
$\hspace{5pt}$
$\hspace{5pt}$ **MAJOR.MINOR.PATCH**
$\hspace{5pt}$
$\hspace{5pt}$ Examples:
$\hspace{5pt}$ - `1.0.0` - unheard of in the rust or luau community
$\hspace{5pt}$ - `0.0.1-rc.1`
$\hspace{5pt}$ - `0.2.0+build231`
$\hspace{5pt}$
$\hspace{5pt}$ To adhere to semver, you should follow these rules:
$\hspace{5pt}$ - Increment the **major** version when you make incompatible API changes, or the **minor** version
$\hspace{5pt}$ if your project is < `1.0.0`.
$\hspace{5pt}$ - Increment the **minor** version when you add functionality in a backwards-compatible manner.
$\hspace{5pt}$ - Increment the **patch** version when you make backwards-compatible bug fixes.
$\hspace{5pt}$ - Use a **release candidate** suffix (`-rc.<number>`) to indicate pre-release versions that are not yet stable.
$\hspace{5pt}$ - Use a **build** suffix (`+build123`) to attach metadata that does not affect version precedence.
$\hspace{5pt}$ - Pre-release versions are ordered before their corresponding release: `1.0.0-rc.1 < 1.0.0`.
$\hspace{5pt}$ - Build metadata is ignored when comparing versions: `1.0.0+abc == 1.0.0+xyz`.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$

```luau
local semver = require("@std/semver")
local first_version = semver.from("0.0.1")
local second_version = semver.from("0.0.2")

first_version:satisfies("^0.0.1") --> true
second_version:satisfies("^0.0.1") --> true
assert(first_version < second_version)
$\hspace{5pt}$ ```


SemverFields.major: `number`

SemverFields.minor: `number`

SemverFields.patch: `number`

SemverFields.metadata.release_candidate.tag: `string`

SemverFields.metadata.release_candidate.ver: `number`

SemverFields.metadata.release_candidate.build: `string?`

`export type` SemverImpl

`export type` Semver

Semver.local fields: `SemverFields = {`

.function semver.from(s: `string): Semver`

.error("Invalid semver string: `" .. s)`

.error(`Invalid semver string: `{s}`)`

.local fields: `SemverFields = {`

.metadata = metadata : `: any, -- LUAU FIXME: not optional fields not qualifying as optionals`

`export type` SemverVals

SemverVals.function semver.satisfies(self: `Semver, semver_range: string): boolean`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Returns true if the `self` is compatible with (within the range of) semver_range.
$\hspace{5pt}$ 
$\hspace{5pt}$ `semver_range` supports the following syntaxes:
$\hspace{5pt}$ - `^` like `^0.1.0`, satisfied by any semvers greater than or equal to `0.1.0` but less than `0.2.0`,
$\hspace{5pt}$ - `==` like `==0.1.0` for exact matches,
$\hspace{5pt}$ - `<=` like `<=1.0.0` for upper bounds that are not necessarily equivalent to ^,
$\hspace{5pt}$ - `>` like `<1.0.1` for lower bounds (exclusive),
$\hspace{5pt}$ - Defaults to `^` when no operator provided (`0.2.1` defaults to `^0.2.1`),
$\hspace{5pt}$ - Multiple constraints can be space-separated, e.g. `>=1.2.3 <2.0.0`, which all must be satisfied.
$\hspace{5pt}$ 
$\hspace{5pt}$ Note that release candidates (rc.<number>) are ordered before full releases, therefore
$\hspace{5pt}$ `0.2.0-rc.1` < `0.2.0`.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ 
```luau
local semver = require("@std/semver")

local some_version = semver.from(require("./config.luau").version)
if some_version:satisfies("^0.1.0") then
    print("compatible version!")
else
    print("incompatible version :(")
end
$\hspace{5pt}$ ```

</details>


SemverVals.local function get_specific_range(part: `string): (SemverVals, string)`

SemverVals.function semver.__eq(self: `Semver, other: Semver): boolean`

SemverVals.function semver.__lt(self: `Semver, other: Semver): boolean`

SemverVals.function semver.__le(self: `Semver, other: Semver): boolean`

SemverVals.function semver.__tostring(self: `Semver): string`
