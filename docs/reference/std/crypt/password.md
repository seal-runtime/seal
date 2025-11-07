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

$hspace{5pt}$## This is the password handling lib.
$hspace{5pt}$
$hspace{5pt}$Please use with caution.

Password.hash: `(raw_password: string) -> HashedPassword`

<details>

<summary> See the docs </summary

$hspace{5pt}$Hash a password with the `PBKDF2_HMAC_SHA256` algorithm, returns a `HashedPassword`
$hspace{5pt}$which you can later use to verify the password against a future
$hspace{5pt}$passwording attempt.
$hspace{5pt}$
$hspace{5pt}$## Example:
$hspace{5pt}$```luau
$hspace{5pt}$local input = require("@std/io/input")
$hspace{5pt}$local password = require("@std/crypt/password")
$hspace{5pt}$local raw_password = input.get("enter a password: ")
$hspace{5pt}$
$hspace{5pt}$-- hash a password
$hspace{5pt}$local hashed_password = password.hash(raw_password)
$hspace{5pt}$
$hspace{5pt}$-- verify password
$hspace{5pt}$local password_to_verify = input.get("verify password: ")
$hspace{5pt}$local verify_options = {
$hspace{5pt}$    raw_password = password_to_verify,
$hspace{5pt}$    hashed_password = hashed_password,
$hspace{5pt}$}
$hspace{5pt}$if password.verify(verify_options) then
$hspace{5pt}$    print("right password")
$hspace{5pt}$else
$hspace{5pt}$    print("wrong password")
$hspace{5pt}$end
$hspace{5pt}$```

</details>

Password.verify: `(verify_options: PasswordVerifyOptions) -> boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Takes in a table of type:
$hspace{5pt}$```luau
$hspace{5pt}$type PasswordVerifyOptions = {
$hspace{5pt}$    raw_password: string,
$hspace{5pt}$    hashed_password: HashedPassword,
$hspace{5pt}$}
$hspace{5pt}$type HashedPassword = {
$hspace{5pt}$    salt: buffer,
$hspace{5pt}$    hash: buffer,
$hspace{5pt}$}
$hspace{5pt}$
$hspace{5pt}$```
$hspace{5pt}$and returns a boolean (if valid or not)
$hspace{5pt}$
$hspace{5pt}$## Example:
$hspace{5pt}$```luau
$hspace{5pt}$local input = require("@std/io/input")
$hspace{5pt}$local password = require("@std/crypt/password")
$hspace{5pt}$local raw_password = input.get("enter a password: ")
$hspace{5pt}$
$hspace{5pt}$-- hash a password
$hspace{5pt}$local hashed_password = password.hash(raw_password)
$hspace{5pt}$
$hspace{5pt}$-- verify password
$hspace{5pt}$local password_to_verify = input.get("verify password: ")
$hspace{5pt}$local verify_options = {
$hspace{5pt}$    raw_password = password_to_verify,
$hspace{5pt}$    hashed_password = hashed_password,
$hspace{5pt}$}
$hspace{5pt}$if password.verify(verify_options) then
$hspace{5pt}$    print("right password")
$hspace{5pt}$else
$hspace{5pt}$    print("wrong password")
$hspace{5pt}$end
$hspace{5pt}$```

</details>
