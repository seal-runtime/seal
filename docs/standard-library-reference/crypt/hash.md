<!-- markdownlint-disable MD033 -->

# Hash

`export type Hash = {`

<details>

<summary> See the docs </summary

Contains function sha2, which can be used to create an **unsalted** hash, returned as a buffer.

### Please use the `@std/crypt/password` library if you want to hash passwords (salted)

</details>

`function Hash.sha2(plaintext: string): buffer`

<details>

<summary> See the docs </summary

Hashes plaintext with the SHA2-256 algorithm, returns a buffer (of length 32) containing the result.

You can use the @std/serde library to convert the result into a 'readable' format

</details>
