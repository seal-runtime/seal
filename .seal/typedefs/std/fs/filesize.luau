--[=[
    Construct `FileSize` values from human-readable units rather than raw byte counts.

    Fractional unit counts are accepted for all units larger than bytes — e.g. `filesize.gigabytes(1.2)` gives you 1.2 GB expressed as bytes.

    `FileSize`s may be compared with all comparison operators and added and subtracted.
]=]
export type FileSizeLib = {
    --- Constructs a `FileSize` representing `count` bytes. `count` must be a whole number.
    bytes: (count: number) -> FileSize,

    --- Constructs a `FileSize` representing `count` kilobytes (1 KB = 1,024 bytes).
    kilobytes: (count: number) -> FileSize,

    --- Constructs a `FileSize` representing `count` megabytes (1 MB = 1,024 KB).
    megabytes: (count: number) -> FileSize,

    --- Constructs a `FileSize` representing `count` gigabytes (1 GB = 1,024 MB).
    gigabytes: (count: number) -> FileSize,

    --- Constructs a `FileSize` representing `count` terabytes (1 TB = 1,024 GB).
    terabytes: (count: number) -> FileSize,
}

return {} :: FileSizeLib
