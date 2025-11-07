<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# crypt.password

`local password = require("@std/crypt/password")`

HashedPassword.salt: `buffer`

HashedPassword.hash: `buffer`

`export type` PasswordVerifyOptions

PasswordVerifyOptions.raw_password: `string`

PasswordVerifyOptions.hashed_password: `HashedPassword`

`export type` Password

$\hspace{5pt}$ ## This is the password handling lib.
$\hspace{5pt}$
$\hspace{5pt}$ Please use with caution.

Password.hash: `(raw_password: string) -> HashedPassword`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Hash a password with the `PBKDF2_HMAC_SHA256` algorithm, returns a `HashedPassword`
$\hspace{5pt}$ which you can later use to verify the password against a future
$\hspace{5pt}$ passwording attempt.
$\hspace{5pt}$
$\hspace{5pt}$ ## Example:

```luau
local input = require("@std/io/input")
local password = require("@std/crypt/password")
local raw_password = input.get("enter a password: ")

-- hash a password
local hashed_password = password.hash(raw_password)

-- verify password
local password_to_verify = input.get("verify password: ")
local verify_options = {
    raw_password = password_to_verify,
    hashed_password = hashed_password,
}
if password.verify(verify_options) then
    print("right password")
else
    print("wrong password")
end
$\hspace{5pt}$ ```

</details>


Password.verify: `(verify_options: PasswordVerifyOptions) -> boolean`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Takes in a table of type:
```luau
type PasswordVerifyOptions = {
    raw_password: string,
    hashed_password: HashedPassword,
}
type HashedPassword = {
    salt: buffer,
    hash: buffer,
}

$\hspace{5pt}$ ```
$\hspace{5pt}$ and returns a boolean (if valid or not)
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Example:
```luau
local input = require("@std/io/input")
local password = require("@std/crypt/password")
local raw_password = input.get("enter a password: ")

-- hash a password
local hashed_password = password.hash(raw_password)

-- verify password
local password_to_verify = input.get("verify password: ")
local verify_options = {
    raw_password = password_to_verify,
    hashed_password = hashed_password,
}
if password.verify(verify_options) then
    print("right password")
else
    print("wrong password")
end
$\hspace{5pt}$ ```

</details>

