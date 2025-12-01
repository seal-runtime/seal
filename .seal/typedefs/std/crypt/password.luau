
--[=[
## This is the password handling lib.

Please use with caution.
]=]
export type password = {
    --[=[
    Hash a password with the `PBKDF2_HMAC_SHA256` algorithm, returns a `HashedPassword`
    which you can later use to verify the password against a future
    passwording attempt.

    ## Example:
    ```luau
    local prompt = require("@std/io/prompt")
    local password = require("@std/crypt/password")
    local raw_password = prompt.password("enter a password: ")

    -- hash a password
    local hashed_password = password.hash(raw_password)

    -- verify password
    local password_to_verify = prompt.password("verify password: ")
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
    ]=]
    hash: (raw_password: string) -> HashedPassword,
    --[=[
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

    ## Example:
    ```luau
    local prompt = require("@std/io/prompt")
    local password = require("@std/crypt/password")
    local raw_password = prompt.password("enter a password: ")

    -- hash a password
    local hashed_password = password.hash(raw_password)

    -- verify password
    local password_to_verify = prompt.password("verify password: ")
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
    ]=]
    verify: (verify_options: PasswordVerifyOptions) -> boolean
}

--[=[
    Represents a password hashed with the PBKDF2_HMAC_SHA256 algorithm.

    - `salt` is a pseudorandom buffer used during hashing to make the hash harder to guess.
    - `hash` is the result of the hashing operation.

    ## ⚠️ Safety and Security

    - Don't store or send the raw password string.
    - Use `base64.encode` if you need to save `salt` and `buffer` to text-based storage.
    - Do not log, print, or expose `salt` or `hash` in production (anywhere outside testing)
]=]
export type HashedPassword = {
    salt: buffer,
    hash: buffer,
}

export type PasswordVerifyOptions = {
    raw_password: string,
    hashed_password: HashedPassword,
}

return {} :: password