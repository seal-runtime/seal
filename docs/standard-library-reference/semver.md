<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Semver

### semver = semantic versioning

This implementation roughly follows the described spec at <https://semver.org>.

Basically, a semver consists of 3 components: major, minor, and patch, with an optional
`-rc.<number>` release candidate suffix or `+buildnumber` build suffix:

**MAJOR.MINOR.PATCH**

Examples:

- `1.0.0` - unheard of in the rust or luau community
- `0.0.1-rc.1`
- `0.2.0+build231`

To adhere to semver, you should follow these rules:

- Increment the **major** version when you make incompatible API changes, or the **minor** version
if your project is < `1.0.0`.
- Increment the **minor** version when you add functionality in a backwards-compatible manner.
- Increment the **patch** version when you make backwards-compatible bug fixes.
- Use a **release candidate** suffix (`-rc.<number>`) to indicate pre-release versions that are not yet stable.
- Use a **build** suffix (`+build123`) to attach metadata that does not affect version precedence.
- Pre-release versions are ordered before their corresponding release: `1.0.0-rc.1 < 1.0.0`.
- Build metadata is ignored when comparing versions: `1.0.0+abc == 1.0.0+xyz`.

## Usage

```luau
local semver = require("@std/semver")
local first_version = semver.from("0.0.1")
local second_version = semver.from("0.0.2")

first_version:satisfies("^0.0.1") --> true
second_version:satisfies("^0.0.1") --> true
assert(first_version < second_version)
```

`major: number`

`minor: number`

`patch: number`

`metadata: {`

`release_candidate: {`

`release_candidate.tag: string`

`release_candidate.ver: number`

`release_candidate.}?`

`release_candidate.build: string?`

`}`

`function semver.default(): Semver`

`local fields: SemverFields = {`

`Semver.major = 0`

`Semver.minor = 1`

`Semver.patch = 0`

`Semver.metadata = {}`

`}`

`Semver.return setmetatable(fields, semver)`

`function semver.from(s: string): Semver`

`Semver.local major, minor, patch = string.match(s, "^(%d+)%.(%d+)%.(%d+)")`

`Semver.if not major or not minor or not patch then`

`Semver.error("Invalid semver string: " .. s)`

`Semver.else`

`Semver.major = tonumber(major)`

`Semver.minor = tonumber(minor)`

`Semver.patch = tonumber(patch)`

`Semver.if not major or not minor or not patch then`

`Semver.error(`Invalid semver string: {s}`)`

`Semver.end`

`Semver.end`

`Semver.local metadata = {}`

`Semver.-- Extract release candidate (e.g., -rc.1)`

`Semver.local rc_name, rc_version = string.match(s, "%-(%a+)%.(%d+)")`

`Semver.if rc_name and rc_version then`

`Semver.metadata.release_candidate = {`

`Semver.tag = rc_name`

`Semver.ver = tonumber(rc_version) :: number`

`Semver.}`

`Semver.end`

`Semver.-- Extract build metadata (e.g., +build123)`

`Semver.local build = string.match(s, "%+([%w%.%-]+)")`

`Semver.if build then`

`Semver.metadata.build = build`

`Semver.end`

`Semver.assert(`

`Semver.typeof(major) == "number"`

`Semver.and typeof(minor) == "number"`

`Semver.and typeof(patch) == "number"`

`Semver.`Unexpected major/minor/patch ({major}, {minor}, {patch})``

`)`

`local fields: SemverFields = {`

`Semver.major = major`

`Semver.minor = minor`

`Semver.patch = patch`

`Semver.metadata = metadata :: any, -- LUAU FIXME: not optional fields not qualifying as optionals`

`}`

`Semver.return setmetatable(fields, semver)`

`Semver.["^"] = "AND UP"`

`Semver.["=="] = "EXACTLY EQUAL"`

`Semver.["<="] = "LESS THAN OR EQUAL"`

`Semver.["<"] = "LESS THAN"`

`Semver.[">"] = "GREATER THAN NOT INCLUDING"`

`Semver.| "AND UP"`

`Semver.| "EXACTLY EQUAL"`

`Semver.| "LESS THAN OR EQUAL"`

`Semver.| "LESS THAN"`

`Semver.| "GREATER THAN NOT INCLUDING"`

`function semver.satisfies(self: Semver, semver_range: string): boolean`

<details>

<summary> See the docs </summary

Returns true if the `self` is compatible with (within the range of) semver_range.

`semver_range` supports the following syntaxes:

- `^` like `^0.1.0`, satisfied by any semvers greater than or equal to `0.1.0` but less than `0.2.0`,
- `==` like `==0.1.0` for exact matches,
- `<=` like `<=1.0.0` for upper bounds that are not necessarily equivalent to ^,
- `>` like `<1.0.1` for lower bounds (exclusive),
- Defaults to `^` when no operator provided (`0.2.1` defaults to `^0.2.1`),
- Multiple constraints can be space-separated, e.g. `>=1.2.3 <2.0.0`, which all must be satisfied.

Note that release candidates (rc.<number>) are ordered before full releases, therefore
`0.2.0-rc.1` < `0.2.0`.

## Usage

```luau
    local semver = require("@std/semver")

    local some_version = semver.from(require("./config.luau").version)
    if some_version:satisfies("^0.1.0") then
        print("compatible version!")
    else
        print("incompatible version :(")
    end
```

</details>

`local function get_specific_range(part: string): (SemverVals, string)`

`Semver.for k, _ in semver_ranges :: { [string]: string } do`

`Semver.if str.startswith(part, k) then`

`local op = (semver_ranges :: any)[k] :: SemverVals`

`Semver.local version = str.trimfront(part, k)`

`Semver.return op, version`

`Semver.end`

`Semver.end`

`Semver.return "AND UP", part`

`Semver.end`

`Semver.local parts = string.split(semver_range, " ")`

`Semver.for _, part in parts do`

`Semver.local op, version_str = get_specific_range(part)`

`Semver.local other = semver.from(version_str)`

`Semver.local satisfies = false`

`Semver.if op == "AND UP" then`

`Semver.local upper_bound`

`Semver.if other.major > 0 then`

`Semver.semver.from(`{other.major + 1}.0.0`)`

`Semver.elseif other.minor > 0 then`

`Semver.semver.from(`0.{other.minor + 1}.0`)`

`Semver.else`

`Semver.semver.from(`0.0.{other.patch + 1}`)`

`Semver.satisfies = self >= other and self < upper_bound`

`Semver.elseif op == "EXACTLY EQUAL" then`

`Semver.satisfies = (self :: any) == other -- LUAU FIXME`

`Semver.elseif op == "LESS THAN" then`

`Semver.satisfies = self < other`

`Semver.elseif op == "GREATER THAN NOT INCLUDING" then`

`Semver.satisfies = self > other`

`Semver.else`

`Semver.satisfies = false`

`Semver.end`

`Semver.if not satisfies then`

`Semver.return false`

`Semver.end`

`Semver.end`

`Semver.return true`

`function semver.__eq(self: Semver, other: Semver): boolean`

`Semver.if`

`Semver.self.major ~= other.major or`

`Semver.self.minor ~= other.minor or`

`Semver.self.patch ~= other.patch`

`Semver.then`

`Semver.return false`

`Semver.end`

`Semver.local a_rc = self.metadata.release_candidate`

`Semver.local b_rc = other.metadata.release_candidate`

`Semver.if (a_rc == nil) ~= (b_rc == nil) then`

`Semver.return false`

`Semver.elseif a_rc and b_rc then`

`Semver.if a_rc.tag ~= b_rc.tag or a_rc.ver ~= b_rc.ver then`

`Semver.return false`

`Semver.end`

`Semver.end`

`Semver.if self.metadata.build ~= other.metadata.build then`

`Semver.return false`

`Semver.end`

`Semver.return true`

`function semver.__lt(self: Semver, other: Semver): boolean`

`Semver.if self.major ~= other.major then -- self.major and other.major hovers are number`

`Semver.return self.major < other.major`

`Semver.end`

`Semver.if self.minor ~= other.minor then -- self.minor hover is any`

`Semver.return self.minor < other.minor`

`Semver.end`

`Semver.if self.patch ~= other.patch then`

`Semver.return self.patch < other.patch`

`Semver.end`

`Semver.local a_rc = self.metadata.release_candidate -- why tf is everything here any`

`Semver.local b_rc = other.metadata.release_candidate`

`Semver.-- Pre-release versions are lower than normal versions`

`Semver.if a_rc == nil and b_rc ~= nil then`

`Semver.return false`

`Semver.elseif a_rc ~= nil and b_rc == nil then`

`Semver.return true`

`Semver.elseif a_rc and b_rc then`

`Semver.if a_rc.tag ~= b_rc.tag then`

`Semver.return a_rc.tag < b_rc.tag -- lexicographic comparison`

`Semver.end`

`Semver.return a_rc.ver < b_rc.ver -- numeric comparison`

`Semver.end`

`Semver.-- build metadata is ignored for ordering`

`Semver.return false`

`function semver.__le(self: Semver, other: Semver): boolean`

`Semver.if self.major ~= other.major then`

`Semver.return self.major <= other.major`

`Semver.end`

`Semver.if self.minor ~= other.minor then`

`Semver.return self.minor <= other.minor`

`Semver.end`

`Semver.if self.patch ~= other.patch then`

`Semver.return self.patch <= other.patch`

`Semver.end`

`Semver.local a_rc = self.metadata.release_candidate`

`Semver.local b_rc = other.metadata.release_candidate`

`Semver.-- Pre-release versions are lower than normal versions`

`Semver.if a_rc == nil and b_rc ~= nil then`

`Semver.return false`

`Semver.elseif a_rc ~= nil and b_rc == nil then`

`Semver.return true`

`Semver.elseif a_rc and b_rc then`

`Semver.if a_rc.tag ~= b_rc.tag then`

`Semver.return a_rc.tag <= b_rc.tag -- lexicographic comparison`

`Semver.end`

`Semver.return a_rc.ver <= b_rc.ver -- numeric comparison`

`Semver.end`

`Semver.-- Build metadata is ignored for ordering`

`Semver.return true`

`function semver.__tostring(self: Semver): string`

`Semver.local result =`{self.major}.{self.minor}.{self.patch}``

`Semver.if self.metadata.release_candidate then`

`Semver.result ..=`-{self.metadata.release_candidate.tag}.{self.metadata.release_candidate.ver}``

`Semver.end`

`Semver.if self.metadata.build then`

`Semver.result ..=`+{self.metadata.build}``

`Semver.end`

`Semver.return result`
