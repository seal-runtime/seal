<!-- markdownlint-disable MD033 -->

# Password

`export type Password = {`

<details>

<summary> Docs </summary

## This is the password handling lib

Please use with caution.

</details>

`function Password.hash(raw_password: string): HashedPassword`

<details>

<summary> Docs </summary

Hash a password with the `PBKDF2_HMAC_SHA256` algorithm, returns a `HashedPassword`
which you can later use to verify the password against a future
passwording attempt.

## Example

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
```

</details>

`function Password.verify(verify_options: PasswordVerifyOptions): boolean`

<details>

<summary> Docs </summary

Takes in a table of type:

```luau
 type PasswordVerifyOptions = {
  raw_password: string,
  hashed_password: HashedPassword,
 }
 type HashedPassword = {
  salt: buffer,
  hash: buffer,
 }

```

and returns a boolean (if valid or not)

## Example

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
```

</details>
