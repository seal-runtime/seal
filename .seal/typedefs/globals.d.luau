declare function pcall<T, Args...>(protected_callback: ((...any) -> ...T), ...: Args...): (boolean, T | error)

declare function ecall<T, A...>(f: (A...) -> T | error): (A...) -> T

declare function p<T...>(...: T...): T...

declare function pp<T...>(...: T...): T...

declare function dp<T...>(...: T...): T...

declare function warn<T...>(...: T...): ()

declare _SEAL_VERSION: string

--[=[
    Information about the seal runtime, conforming to the cross-runtime
    [`_RUNTIME` / `_LUAU` specification](https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf).
    
    All fields are read-only (frozen). `_RUNTIME` is always present;
    `_LUAU` is always present in seal but the spec permits its omission
    in other runtimes — check before use in portable code.
]=]
declare _RUNTIME: {
    --- Always `"seal"`.
    read name: "seal",
    --- Version of the seal runtime itself.
    read version: {
        --- Stringified version, e.g. `"0.0.8-rc.2"`.
        read display: string,
        --- Semantic version components.
        read semantic: {
            read major: number,
            read minor: number,
            read patch: number,
            read prerelease: string?,
            read build: string?,
        },
        --- Git revision seal was built from.
        read git: {
            read url: string,
            read commit: string,
            read branch: string,
        },
    },
    --- Link to the seal repository.
    read url: string,
    --- seal-specific extra metadata (spec `extra` field).
    read extra: {
        --- Whether seal was compiled with Luau JIT (NCG) support. Will only be false on Android.
        read jit: boolean,
        --- `true` when seal was compiled in debug mode, `false` for release builds.
        read debug: boolean,
    },
}

--[=[
    Information about the Luau version embedded in seal, conforming to the cross-runtime
    [`_RUNTIME` / `_LUAU` specification](https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf).
]=]
declare _LUAU: {
    --- Luau version information.
    read version: {
        --- Stringified Luau version, e.g. `"0.722"`.
        read display: string,
        --- Major component (currently always `0`).
        read major: number,
        --- Minor component, e.g. `722`.
        read minor: number,
    },
    --- Link to the upstream Luau repository.
    read url: string,
}

declare script: {
    entry_path: string,
    context: string?,
    path: (self: any) -> string,
    parent: (self: any) -> string,
    project: (self: any) -> string,
}

declare channel: {
    send: <D>(self: any, data: D | string) -> (),
    sendbytes: <D>(self: any, data: buffer) -> (),
    read: <D>(self: any) -> D?,
    read_await: <D>(self: any) -> D,
    readbytes: (self: any) -> buffer?,
    readbytes_await: (self: any) -> buffer,
    data: any?,
}?

--- an error returned from mluau, can be tostringed
declare extern type error with end

--- Represents the JSON `null`, which can be stored in tables.
declare extern type null with end

declare extern type interrupt with
    code: "CtrlC" | "CtrlD"
end

type ContainerFormat =
    | "Zip"
    | "Tar"
    | "Ar"
    | "Deb"
    | "TarGz"
    | "TarBz2"
    | "TarXz"
    | "TarZst"
    | "TarLz4"
    | "7z"

type SingleFileFormat =
    | "Gz"
    | "Bz2"
    | "Xz"
    | "Lz4"
    | "Zst"

type ArchiveFormat = ContainerFormat | SingleFileFormat

type ArchiveOptions = {
    max_file_size: FileSize?,
    max_total_size: FileSize?,
    allow_symlinks: boolean?,
    allow_unsafe_path_traversals: boolean?,
    compression_level: CompressionLevel?,
}

--[=[
    Represents an individual entry in an `Archive`.

    Right now, the 3 supported kinds of entries are Files, Directories, and Symlinks.
]=]
declare extern type ArchiveEntry with
    type: "File" | "Directory" | "Symlink"
    --[=[
        A non-unique string identifier for the entry within the archive.

        Archive paths don't necessarily correspond to filesystem paths, and could be an arbitrary string.
    ]=]
    function path(self): string
    --[=[
        Change the `path` of this entry in-place.
    ]=]
    function set_path(self, path: string): ()
    --[=[
        Gets the last modified time (mtime) of this entry, if present.
    ]=]
    function modified(self): DateTime?
    --[=[
        Set the last modified time (mtime) of this entry.
    ]=]
    function set_modified(self, dt: DateTime): ()
    --[=[
        Gets the unix permission bits (mode) of this entry, if present.
    ]=]
    function unix_mode(self): number?
    --[=[
        Set the unix permission bits (mode) of this entry, if present.
    ]=]
    function set_unix_mode(self, mode: number): ()
    --[=[
        If this entry represents a File, gets the file's contents' size as
        a `FileSize` instance (from @std/fs/filesize).

        Returns `nil` if this entry doesn't represent a File.
    ]=]
    function size(self): FileSize?

    --[=[
        Narrow this into an `ArchiveEntryFile` if this entry represents a File.
    ]=]
    function try_file(self): ArchiveEntryFile?
    --[=[
        Expect/assert that this entry should be an `ArchiveEntryFile`, throwing an error if it isn't.
    ]=]
    function expect_file(self): ArchiveEntryFile

    --[=[
        Narrow this into an `ArchiveEntryDirectory` if this entry represents a Directory.
    ]=]
    function try_directory(self): ArchiveEntryDirectory?
    --[=[
        Expect/assert that this entry should be an `ArchiveEntryDirectory`, throwing an error if it isn't.
    ]=]
    function expect_directory(self): ArchiveEntryDirectory

    --[=[
        Narrow this into an `ArchiveEntrySymlink` if this entry represents a Symlink.
    ]=]
    function try_symlink(self): ArchiveEntrySymlink?
    --[=[
        Expect/assert that this entry should be an `ArchiveEntrySymlink`, throwing an error if it isn't.
    ]=]
    function expect_symlink(self): ArchiveEntrySymlink
    --[=[
        Returns a human-readable summary of this entry, in the format
        `ArchiveEntry<File: "dir/hello.txt", 20.23 MB, modified: 2025-08-18 18:20 [CDT]>`.

        Used automatically by `print`/`pp` when displaying an `ArchiveEntry`.
    ]=]
    function display(self): string
end

--[=[
    A regular file, or rarely, a special file (such as a char/block device, FIFO, socket, etc.) mistakenly
    caught as a regular file. 7z symlinks also appear as `ArchiveEntryFile` due to a limitation in an underlying
    dependency.
]=]
declare extern type ArchiveEntryFile extends ArchiveEntry with
    type: "File"
    --[=[
        Copies file's contents into a new string exposed to Luau.
    ]=]
    function contents(self): string
    --[=[
        Changes the file's contents in-place to the provided string or buffer.
    ]=]
    function replace_contents(self, contents: string | buffer): ()
    --[=[
        Gets the size of this entry's `:contents()` as a `FileSize`
        instance (from @std/fs/filesize).
    ]=]
    function size(self): FileSize
    --[=[
        Compresses this entry into a buffer using a `SingleFileFormat` codec,
        such as `Gzip`, `Zst`, etc.
    ]=]
    function compress(self, format: SingleFileFormat): buffer
    --[=[
        Writes this file's contents straight to `destination` on disk (creating parent
        directories as needed), applying this entry's mode and mtime if present.

        Unlike `:contents()`, this never copies the file's data into a Luau string/buffer.
    ]=]
    function extract(self, destination: string): ()
end

--[=[
    A directory in an archive can basically just represent an empty directory,
    you don't need an `ArchiveEntryDirectory` if you have file paths within the directory.
]=]
declare extern type ArchiveEntryDirectory extends ArchiveEntry with
    type: "Directory"
end

--[=[
    A symlink points from place A (`path`) to place B (`target`). 
    Symlinks within Archives should point within the same archive,
    otherwise we throw unsafe_path_traversal errors (which you can disable).

    7z symlinks are incorrectly caught as `ArchiveEntryFile` instead of this due to a limitation in
    a dependency.

    Hardlinks are also caught as `ArchiveEntrySymlink`s.
]=]
declare extern type ArchiveEntrySymlink extends ArchiveEntry with
    type: "Symlink"
    function target(self): string
    function set_target(self, target: string): ()
end

--[=[
    Represents any compressed archive (tar.gz, Zip file, 7z, Debian packages/etc.)

    The `Archive` type itself is format-agnostic and can be re-saved in a different format.

    To see what's in the archive, for loop it, try printing it, or call`:paths()`.

    To get the length of an archive, just use `#archive`.

    Please don't try and assume that all paths are unique because most archive types can have many entries
    with the same path (unlike a real filesystem). Attackers have been known to exploit that before.
]=]
declare extern type Archive with
    --[=[
        Returns a list of paths in the archive; use indices to pass into `Archive:get` or `Archive:remove`.

        Keep in mind that an archive may contain duplicate paths or paths that don't follow filesystem
        path conventions and cannot be written to disk.
    ]=]
    function paths(self): { string }
    --[=[
        If passed an index, gets the `ArchiveEntry` at a specific index in the archive.
        If passed a path, gets the first `ArchiveEntry` with the requested path.
        
        Returns `nil` if no entry matching `path_or_index` is found in the archive.

        If a path is specified (a string), gets the first entry with that path. Prefer using indices
        instead of paths unless you know all paths are unique.
    ]=]
    function get(self, path_or_index: string | number): ArchiveEntry?
    --[=[
        If passed an index, gets the `ArchiveEntry` at a specific index in the archive.
        If passed a path, gets the first `ArchiveEntry` with the requested path.

        Unlike `Archive:get`, throws an error instead of returning `nil` if no entry matching `path_or_index`
        is found in the archive.
    ]=]
    function expect(self, path_or_index: string | number): ArchiveEntry
    --[=[
        Adds a single `ArchiveEntry` to an existing archive in place with similar semantics to `table.insert`.
        Create entries from `@std/archive/entry`.
    ]=]
    function insert(self, index: number, entry: ArchiveEntry | string)
    --[=[
        Adds one or more entries to the back of an archive.
        Create entries from `@std/archive/entry`.
    ]=]
    function append(self, entry: ArchiveEntry | { ArchiveEntry }, ...: ArchiveEntry | { ArchiveEntry })
    --[=[
        Removes the `ArchiveEntry` element by index, pulling all remaining entries towards the front.

        This function only takes an index because an archive can contain multiple identical paths
        and removal by path would therefore be a footgun.

        Use `:paths()` to get indices to remove.
    ]=]
    function remove(self, index: number)
    --[=[
        Compress the archive into a buffer of the requested archive or single file format.

        You can just save this via `fs.readfile`. You could also use any archive library's
        writefile function to write this directly to disk without needing to create an additional buffer.`

        If you just want to take an existing archive that exists on disk and extract it, use one of
        the archive libraries' extract functions instead (`archive.extract`/etc. stream directly to disk
        without as many allocations).
    ]=]
    function serialize(self, format: ArchiveFormat, options: ArchiveOptions?): buffer
    --[=[
        Extract this archive into the directory at `destination`, creating a new directory there if needed.

        If this archive represents a single file in a compressed-file format, you probably want to get the
        first entry and `fs.writefile` its `:contents()` instead (this function will make a new directory
        at `destination` and put the file in there).
    ]=]
    function extract(self, destination: string, options: ArchiveOptions?)

    function __len(self): number
    function __iter(self): () -> (number, ArchiveEntry)
end

--[=[
    Compression options for Zip (Deflate), Gzip, Bz2, Xz, Zstd, and Lz4-compressed archives.

    You can create these by calling the `.compression` functions on relevant `@std/archive` libraries.

    Passing a compression level that is inapplicable for the requested format or one that's out of range
    for that compression codec will throw a runtime error upon write/serialization.
]=]
declare extern type CompressionLevel with
    --[=[
        Which compression codec this `CompressionLevel` refers to.
    ]=]
    function name(self): string
    --[=[
        The compression level (if the codec supports one)
    ]=]
    function level(self): number?
    function __tostring(self): string
end

--[=[
    Represents the size of data in bytes, but in a more human-readable way with convenient constructors.

    Use the `@std/fs/filesize` library to construct `FileSize`s. This type may be added, subtracted, multiplied, divided, and compared
    using all standard comparison operators. Multiplication and division accept either another `FileSize` or a `number`.

    This type internally stores bytes as `u64`, therefore the largest size this type
    can represent is `u64::MAX` (~16 exabytes)
]=]
declare extern type FileSize with
    function __add(self, other: FileSize): FileSize
    function __sub(self, other: FileSize): FileSize
    function __mul(self, other: FileSize | number): FileSize
    function __div(self, other: FileSize | number): FileSize
    function __eq(self, other: FileSize): boolean
    function __lt(self, other: FileSize): boolean
    function __le(self, other: FileSize): boolean
    function __tostring(self): string
    --- Returns a human-readable string using the most appropriate unit representation.
    function display(self): string
    --- Format the `FileSize` as a human-readable string in the given unit ("1.20 GB"`, `"1234 bytes"`, etc.).
    function format(self, unit: "Bytes" | "KB" | "MB" | "GB" | "TB"): string
    --- Returns the size as a raw byte count.
    function as_bytes(self): number
    --- Returns the size expressed in kilobytes.
    function as_kilobytes(self): number
    --- Returns the size expressed in megabytes.
    function as_megabytes(self): number
    --- Returns the size expressed in gigabytes.
    function as_gigabytes(self): number
    --- Returns the size expressed in terabytes.
    function as_terabytes(self): number
end

declare extern type WebsocketMessage with
    --- Returns `true` if `message:type() == "Binary"`
    function is_binary(self): boolean
    --- Returns `true` if `message:type() == "Text"`
    function is_utf8(self): boolean
    --- Returns the internal `Tungstenite::Message` type of the message
    function type(self): "Binary" | "Text" | "Close" | "Ping" | "Pong" | "Frame"
    --- Returns the message's contents as a `string` without any intermediate utf8 coercion.
    function as_string(self): string
    --- Returns the message's contents as a `buffer`
    function as_bytes(self): buffer
    --[=[
        Try to parse the message as JSON, returning an `error` if decoding the message was unsuccessful.

        ## Usage

        ```luau
        type ReturnType = {
            field1: string,
        }
        local message = websocket:read()
        if message:is_utf8() then
            local contents = message:try_json<<ReturnType>>()
            if typeof(contents) == "error" then
                print("not a json")
            else
                print(contents.field1) -- no type error
            end
        end
        ```
    ]=]
    try_json: <T>(self: WebsocketMessage) -> T | error
    --[=[
        Converts message to json of the provided `T`

        ## Errors

        - if the message is actually binary
        - if the message cannot be decoded from json
    ]=]
    expect_json: <T>(self: WebsocketMessage) -> T
end

type WebsocketCloseCode =
    | "Normal"
    | "Away"
    | "Protocol"
    | "Unsupported"
    | "Status"
    | "Abnormal"
    | "Invalid"
    | "Policy"
    | "Size"
    | "Extension"
    | "Error"
    | "Tls"
    | number

--- weebsocket
declare extern type Websocket with
    --[=[
        Read a (stringified) message from the websocket, blocking until a message is read.

        If the message is json, you'll want to decode it explicitly with `json.decode`, handling any error cases.
    ]=]
    function read(self): WebsocketMessage
    --[=[
        Send a `message` through the websocket.

        The message can be a `string`, a `buffer` (for binary data), or a table, which will be serialized to json (similarly to http.post/etc)
    ]=]
    function send(self, message: string | { [any]: any } | buffer): ()
    --- Returns `true` if we can currently read from the websocket.
    function readable(self): boolean
    --- Returns `true` if we can currently write to the websocket.
    function sendable(self): boolean
    function close(self, code: WebsocketCloseCode?, reason: string?): ()
end

type COMMON_FORMATS =
    | "ISO_8601"
    | "RFC_2822"
    | "RFC_3339"
    | "SHORT_DATE"
    | "SHORT_TIME"
    | "FULL_DATE_TIME"
    | "LOGGING_24_HR"
    | "LOGGING_12_HR"
    | "MM/DD/YY"
    | "MM/DD/YYYY HH:MM (AM/PM)"
    | "MM/DD/YY"
    | "HH:MM (AM/PM)"
    | "AMERICAN_FULL_DATE_TIME"

declare extern type DateTime with
    year: number
    month: number
    day: number
    hour: number
    minute: number
    second: number
    millisecond: number
    weekday: string
    unix_timestamp: number
    timezone: string
    iso: string

    function timespan(self, other: DateTime): TimeSpan
    function to(self, other: DateTime): TimeSpan
    function since(self, other: DateTime): TimeSpan
    function in_timezone(self, timezone: string | IanaTimezone): DateTime

    function format(self, format: string | COMMON_FORMATS): string
    function display(self): string
    function rfc_2822(self): string
    function rfc_3339(self): string
    function rfc_3339_opts(self): string
    function iso_8601(self): string
    function http_date(self): string

    function __add(self, span: TimeSpan): DateTime
    function __sub(self, span: TimeSpan): DateTime
    function __eq(self, other: DateTime): boolean
    function __lt(self, other: DateTime): boolean
    function __le(self, other: DateTime): boolean
end

declare extern type TimeSpan with
    relative_to: DateTime?

    function duration(self): Duration
    function units(self): {
        read years: number,
        read months: number,
        read days: number,
        read hours: number,
        read minutes: number,
        read seconds: number,
        read milliseconds: number,
        read microseconds: number
    }
    function __add(self, other: TimeSpan): TimeSpan
    function __sub(self, other: TimeSpan): TimeSpan
end

declare extern type TerminalAction with
    function execute(self): ()
end

declare extern type Duration with
    years: number
    months: number
    days: number
    hours: number
    minutes: number
    seconds: number
    milliseconds: number
    microseconds: number
    nanoseconds: number

    function abs(self): Duration
    function __add(self, other: Duration): Duration
    function __sub(self, other: Duration): Duration
    function __mul(self, other: Duration | number): Duration
    function __div(self, other: Duration | number): Duration
    function __eq(self, other: Duration): boolean
    function __lt(self, other: Duration): boolean
    function __le(self, other: Duration): boolean
    function display(self): string
end


type IanaTimezone =
    | "Africa/Abidjan"
    | "Africa/Accra"
    | "Africa/Addis_Ababa"
    | "Africa/Algiers"
    | "Africa/Asmara"
    | "Africa/Asmera"
    | "Africa/Bamako"
    | "Africa/Bangui"
    | "Africa/Banjul"
    | "Africa/Bissau"
    | "Africa/Blantyre"
    | "Africa/Brazzaville"
    | "Africa/Bujumbura"
    | "Africa/Cairo"
    | "Africa/Casablanca"
    | "Africa/Ceuta"
    | "Africa/Conakry"
    | "Africa/Dakar"
    | "Africa/Dar_es_Salaam"
    | "Africa/Djibouti"
    | "Africa/Douala"
    | "Africa/El_Aaiun"
    | "Africa/Freetown"
    | "Africa/Gaborone"
    | "Africa/Harare"
    | "Africa/Johannesburg"
    | "Africa/Juba"
    | "Africa/Kampala"
    | "Africa/Khartoum"
    | "Africa/Kigali"
    | "Africa/Kinshasa"
    | "Africa/Lagos"
    | "Africa/Libreville"
    | "Africa/Lome"
    | "Africa/Luanda"
    | "Africa/Lubumbashi"
    | "Africa/Lusaka"
    | "Africa/Malabo"
    | "Africa/Maputo"
    | "Africa/Maseru"
    | "Africa/Mbabane"
    | "Africa/Mogadishu"
    | "Africa/Monrovia"
    | "Africa/Nairobi"
    | "Africa/Ndjamena"
    | "Africa/Niamey"
    | "Africa/Nouakchott"
    | "Africa/Ouagadougou"
    | "Africa/Porto-Novo"
    | "Africa/Sao_Tome"
    | "Africa/Timbuktu"
    | "Africa/Tripoli"
    | "Africa/Tunis"
    | "Africa/Windhoek"
    | "America/Adak"
    | "America/Anchorage"
    | "America/Anguilla"
    | "America/Antigua"
    | "America/Araguaina"
    | "America/Argentina/Buenos_Aires"
    | "America/Argentina/Catamarca"
    | "America/Argentina/ComodRivadavia"
    | "America/Argentina/Cordoba"
    | "America/Argentina/Jujuy"
    | "America/Argentina/La_Rioja"
    | "America/Argentina/Mendoza"
    | "America/Argentina/Rio_Gallegos"
    | "America/Argentina/Salta"
    | "America/Argentina/San_Juan"
    | "America/Argentina/San_Luis"
    | "America/Argentina/Tucuman"
    | "America/Argentina/Ushuaia"
    | "America/Aruba"
    | "America/Asuncion"
    | "America/Atikokan"
    | "America/Atka"
    | "America/Bahia"
    | "America/Bahia_Banderas"
    | "America/Barbados"
    | "America/Belem"
    | "America/Belize"
    | "America/Blanc-Sablon"
    | "America/Boa_Vista"
    | "America/Bogota"
    | "America/Boise"
    | "America/Buenos_Aires"
    | "America/Cambridge_Bay"
    | "America/Campo_Grande"
    | "America/Cancun"
    | "America/Caracas"
    | "America/Catamarca"
    | "America/Cayenne"
    | "America/Cayman"
    | "America/Chicago"
    | "America/Chihuahua"
    | "America/Ciudad_Juarez"
    | "America/Coral_Harbour"
    | "America/Cordoba"
    | "America/Costa_Rica"
    | "America/Creston"
    | "America/Cuiaba"
    | "America/Curacao"
    | "America/Danmarkshavn"
    | "America/Dawson"
    | "America/Dawson_Creek"
    | "America/Denver"
    | "America/Detroit"
    | "America/Dominica"
    | "America/Edmonton"
    | "America/Eirunepe"
    | "America/El_Salvador"
    | "America/Ensenada"
    | "America/Fort_Nelson"
    | "America/Fort_Wayne"
    | "America/Fortaleza"
    | "America/Glace_Bay"
    | "America/Godthab"
    | "America/Goose_Bay"
    | "America/Grand_Turk"
    | "America/Grenada"
    | "America/Guadeloupe"
    | "America/Guatemala"
    | "America/Guayaquil"
    | "America/Guyana"
    | "America/Halifax"
    | "America/Havana"
    | "America/Hermosillo"
    | "America/Indiana/Indianapolis"
    | "America/Indiana/Knox"
    | "America/Indiana/Marengo"
    | "America/Indiana/Petersburg"
    | "America/Indiana/Tell_City"
    | "America/Indiana/Vevay"
    | "America/Indiana/Vincennes"
    | "America/Indiana/Winamac"
    | "America/Indianapolis"
    | "America/Inuvik"
    | "America/Iqaluit"
    | "America/Jamaica"
    | "America/Jujuy"
    | "America/Juneau"
    | "America/Kentucky/Louisville"
    | "America/Kentucky/Monticello"
    | "America/Knox_IN"
    | "America/Kralendijk"
    | "America/La_Paz"
    | "America/Lima"
    | "America/Los_Angeles"
    | "America/Louisville"
    | "America/Lower_Princes"
    | "America/Maceio"
    | "America/Managua"
    | "America/Manaus"
    | "America/Marigot"
    | "America/Martinique"
    | "America/Matamoros"
    | "America/Mazatlan"
    | "America/Mendoza"
    | "America/Menominee"
    | "America/Merida"
    | "America/Metlakatla"
    | "America/Mexico_City"
    | "America/Miquelon"
    | "America/Moncton"
    | "America/Monterrey"
    | "America/Montevideo"
    | "America/Montreal"
    | "America/Montserrat"
    | "America/Nassau"
    | "America/New_York"
    | "America/Nipigon"
    | "America/Nome"
    | "America/Noronha"
    | "America/North_Dakota/Beulah"
    | "America/North_Dakota/Center"
    | "America/North_Dakota/New_Salem"
    | "America/Nuuk"
    | "America/Ojinaga"
    | "America/Panama"
    | "America/Pangnirtung"
    | "America/Paramaribo"
    | "America/Phoenix"
    | "America/Port-au-Prince"
    | "America/Port_of_Spain"
    | "America/Porto_Acre"
    | "America/Porto_Velho"
    | "America/Puerto_Rico"
    | "America/Punta_Arenas"
    | "America/Rainy_River"
    | "America/Rankin_Inlet"
    | "America/Recife"
    | "America/Regina"
    | "America/Resolute"
    | "America/Rio_Branco"
    | "America/Rosario"
    | "America/Santa_Isabel"
    | "America/Santarem"
    | "America/Santiago"
    | "America/Santo_Domingo"
    | "America/Sao_Paulo"
    | "America/Scoresbysund"
    | "America/Shiprock"
    | "America/Sitka"
    | "America/St_Barthelemy"
    | "America/St_Johns"
    | "America/St_Kitts"
    | "America/St_Lucia"
    | "America/St_Thomas"
    | "America/St_Vincent"
    | "America/Swift_Current"
    | "America/Tegucigalpa"
    | "America/Thule"
    | "America/Thunder_Bay"
    | "America/Tijuana"
    | "America/Toronto"
    | "America/Tortola"
    | "America/Vancouver"
    | "America/Virgin"
    | "America/Whitehorse"
    | "America/Winnipeg"
    | "America/Yakutat"
    | "America/Yellowknife"
    | "Antarctica/Casey"
    | "Antarctica/Davis"
    | "Antarctica/DumontDUrville"
    | "Antarctica/Macquarie"
    | "Antarctica/Mawson"
    | "Antarctica/McMurdo"
    | "Antarctica/Palmer"
    | "Antarctica/Rothera"
    | "Antarctica/South_Pole"
    | "Antarctica/Syowa"
    | "Antarctica/Troll"
    | "Antarctica/Vostok"
    | "Arctic/Longyearbyen"
    | "Asia/Aden"
    | "Asia/Almaty"
    | "Asia/Amman"
    | "Asia/Anadyr"
    | "Asia/Aqtau"
    | "Asia/Aqtobe"
    | "Asia/Ashgabat"
    | "Asia/Ashkhabad"
    | "Asia/Atyrau"
    | "Asia/Baghdad"
    | "Asia/Bahrain"
    | "Asia/Baku"
    | "Asia/Bangkok"
    | "Asia/Barnaul"
    | "Asia/Beirut"
    | "Asia/Bishkek"
    | "Asia/Brunei"
    | "Asia/Calcutta"
    | "Asia/Chita"
    | "Asia/Choibalsan"
    | "Asia/Chongqing"
    | "Asia/Chungking"
    | "Asia/Colombo"
    | "Asia/Dacca"
    | "Asia/Damascus"
    | "Asia/Dhaka"
    | "Asia/Dili"
    | "Asia/Dubai"
    | "Asia/Dushanbe"
    | "Asia/Famagusta"
    | "Asia/Gaza"
    | "Asia/Harbin"
    | "Asia/Hebron"
    | "Asia/Ho_Chi_Minh"
    | "Asia/Hong_Kong"
    | "Asia/Hovd"
    | "Asia/Irkutsk"
    | "Asia/Istanbul"
    | "Asia/Jakarta"
    | "Asia/Jayapura"
    | "Asia/Jerusalem"
    | "Asia/Kabul"
    | "Asia/Kamchatka"
    | "Asia/Karachi"
    | "Asia/Kashgar"
    | "Asia/Kathmandu"
    | "Asia/Katmandu"
    | "Asia/Khandyga"
    | "Asia/Kolkata"
    | "Asia/Krasnoyarsk"
    | "Asia/Kuala_Lumpur"
    | "Asia/Kuching"
    | "Asia/Kuwait"
    | "Asia/Macao"
    | "Asia/Macau"
    | "Asia/Magadan"
    | "Asia/Makassar"
    | "Asia/Manila"
    | "Asia/Muscat"
    | "Asia/Nicosia"
    | "Asia/Novokuznetsk"
    | "Asia/Novosibirsk"
    | "Asia/Omsk"
    | "Asia/Oral"
    | "Asia/Phnom_Penh"
    | "Asia/Pontianak"
    | "Asia/Pyongyang"
    | "Asia/Qatar"
    | "Asia/Qostanay"
    | "Asia/Qyzylorda"
    | "Asia/Rangoon"
    | "Asia/Riyadh"
    | "Asia/Saigon"
    | "Asia/Sakhalin"
    | "Asia/Samarkand"
    | "Asia/Seoul"
    | "Asia/Shanghai"
    | "Asia/Singapore"
    | "Asia/Srednekolymsk"
    | "Asia/Taipei"
    | "Asia/Tashkent"
    | "Asia/Tbilisi"
    | "Asia/Tehran"
    | "Asia/Tel_Aviv"
    | "Asia/Thimbu"
    | "Asia/Thimphu"
    | "Asia/Tokyo"
    | "Asia/Tomsk"
    | "Asia/Ujung_Pandang"
    | "Asia/Ulaanbaatar"
    | "Asia/Ulan_Bator"
    | "Asia/Urumqi"
    | "Asia/Ust-Nera"
    | "Asia/Vientiane"
    | "Asia/Vladivostok"
    | "Asia/Yakutsk"
    | "Asia/Yangon"
    | "Asia/Yekaterinburg"
    | "Asia/Yerevan"
    | "Atlantic/Azores"
    | "Atlantic/Bermuda"
    | "Atlantic/Canary"
    | "Atlantic/Cape_Verde"
    | "Atlantic/Faeroe"
    | "Atlantic/Faroe"
    | "Atlantic/Jan_Mayen"
    | "Atlantic/Madeira"
    | "Atlantic/Reykjavik"
    | "Atlantic/South_Georgia"
    | "Atlantic/St_Helena"
    | "Atlantic/Stanley"
    | "Australia/ACT"
    | "Australia/Adelaide"
    | "Australia/Brisbane"
    | "Australia/Broken_Hill"
    | "Australia/Canberra"
    | "Australia/Currie"
    | "Australia/Darwin"
    | "Australia/Eucla"
    | "Australia/Hobart"
    | "Australia/LHI"
    | "Australia/Lindeman"
    | "Australia/Lord_Howe"
    | "Australia/Melbourne"
    | "Australia/NSW"
    | "Australia/North"
    | "Australia/Perth"
    | "Australia/Queensland"
    | "Australia/South"
    | "Australia/Sydney"
    | "Australia/Tasmania"
    | "Australia/Victoria"
    | "Australia/West"
    | "Australia/Yancowinna"
    | "Brazil/Acre"
    | "Brazil/DeNoronha"
    | "Brazil/East"
    | "Brazil/West"
    | "CET"
    | "CST6CDT"
    | "Canada/Atlantic"
    | "Canada/Central"
    | "Canada/Eastern"
    | "Canada/Mountain"
    | "Canada/Newfoundland"
    | "Canada/Pacific"
    | "Canada/Saskatchewan"
    | "Canada/Yukon"
    | "Chile/Continental"
    | "Chile/EasterIsland"
    | "Cuba"
    | "EET"
    | "EST"
    | "EST5EDT"
    | "Egypt"
    | "Eire"
    | "Etc/GMT"
    | "Etc/GMT+0"
    | "Etc/GMT+1"
    | "Etc/GMT+10"
    | "Etc/GMT+11"
    | "Etc/GMT+12"
    | "Etc/GMT+2"
    | "Etc/GMT+3"
    | "Etc/GMT+4"
    | "Etc/GMT+5"
    | "Etc/GMT+6"
    | "Etc/GMT+7"
    | "Etc/GMT+8"
    | "Etc/GMT+9"
    | "Etc/GMT-0"
    | "Etc/GMT-1"
    | "Etc/GMT-10"
    | "Etc/GMT-11"
    | "Etc/GMT-12"
    | "Etc/GMT-13"
    | "Etc/GMT-14"
    | "Etc/GMT-2"
    | "Etc/GMT-3"
    | "Etc/GMT-4"
    | "Etc/GMT-5"
    | "Etc/GMT-6"
    | "Etc/GMT-7"
    | "Etc/GMT-8"
    | "Etc/GMT-9"
    | "Etc/GMT0"
    | "Etc/Greenwich"
    | "Etc/UCT"
    | "Etc/UTC"
    | "Etc/Universal"
    | "Etc/Zulu"
    | "Europe/Amsterdam"
    | "Europe/Andorra"
    | "Europe/Astrakhan"
    | "Europe/Athens"
    | "Europe/Belfast"
    | "Europe/Belgrade"
    | "Europe/Berlin"
    | "Europe/Bratislava"
    | "Europe/Brussels"
    | "Europe/Bucharest"
    | "Europe/Budapest"
    | "Europe/Busingen"
    | "Europe/Chisinau"
    | "Europe/Copenhagen"
    | "Europe/Dublin"
    | "Europe/Gibraltar"
    | "Europe/Guernsey"
    | "Europe/Helsinki"
    | "Europe/Isle_of_Man"
    | "Europe/Istanbul"
    | "Europe/Jersey"
    | "Europe/Kaliningrad"
    | "Europe/Kiev"
    | "Europe/Kirov"
    | "Europe/Kyiv"
    | "Europe/Lisbon"
    | "Europe/Ljubljana"
    | "Europe/London"
    | "Europe/Luxembourg"
    | "Europe/Madrid"
    | "Europe/Malta"
    | "Europe/Mariehamn"
    | "Europe/Minsk"
    | "Europe/Monaco"
    | "Europe/Moscow"
    | "Europe/Nicosia"
    | "Europe/Oslo"
    | "Europe/Paris"
    | "Europe/Podgorica"
    | "Europe/Prague"
    | "Europe/Riga"
    | "Europe/Rome"
    | "Europe/Samara"
    | "Europe/San_Marino"
    | "Europe/Sarajevo"
    | "Europe/Saratov"
    | "Europe/Simferopol"
    | "Europe/Skopje"
    | "Europe/Sofia"
    | "Europe/Stockholm"
    | "Europe/Tallinn"
    | "Europe/Tirane"
    | "Europe/Tiraspol"
    | "Europe/Ulyanovsk"
    | "Europe/Uzhgorod"
    | "Europe/Vaduz"
    | "Europe/Vatican"
    | "Europe/Vienna"
    | "Europe/Vilnius"
    | "Europe/Volgograd"
    | "Europe/Warsaw"
    | "Europe/Zagreb"
    | "Europe/Zaporozhye"
    | "Europe/Zurich"
    | "GB"
    | "GB-Eire"
    | "GMT"
    | "GMT+0"
    | "GMT-0"
    | "GMT0"
    | "Greenwich"
    | "HST"
    | "Hongkong"
    | "Iceland"
    | "Indian/Antananarivo"
    | "Indian/Chagos"
    | "Indian/Christmas"
    | "Indian/Cocos"
    | "Indian/Comoro"
    | "Indian/Kerguelen"
    | "Indian/Mahe"
    | "Indian/Maldives"
    | "Indian/Mauritius"
    | "Indian/Mayotte"
    | "Indian/Reunion"
    | "Iran"
    | "Israel"
    | "Jamaica"
    | "Japan"
    | "Kwajalein"
    | "Libya"
    | "MET"
    | "MST"
    | "MST7MDT"
    | "Mexico/BajaNorte"
    | "Mexico/BajaSur"
    | "Mexico/General"
    | "NZ"
    | "NZ-CHAT"
    | "Navajo"
    | "PRC"
    | "PST8PDT"
    | "Pacific/Apia"
    | "Pacific/Auckland"
    | "Pacific/Bougainville"
    | "Pacific/Chatham"
    | "Pacific/Chuuk"
    | "Pacific/Easter"
    | "Pacific/Efate"
    | "Pacific/Enderbury"
    | "Pacific/Fakaofo"
    | "Pacific/Fiji"
    | "Pacific/Funafuti"
    | "Pacific/Galapagos"
    | "Pacific/Gambier"
    | "Pacific/Guadalcanal"
    | "Pacific/Guam"
    | "Pacific/Honolulu"
    | "Pacific/Johnston"
    | "Pacific/Kanton"
    | "Pacific/Kiritimati"
    | "Pacific/Kosrae"
    | "Pacific/Kwajalein"
    | "Pacific/Majuro"
    | "Pacific/Marquesas"
    | "Pacific/Midway"
    | "Pacific/Nauru"
    | "Pacific/Niue"
    | "Pacific/Norfolk"
    | "Pacific/Noumea"
    | "Pacific/Pago_Pago"
    | "Pacific/Palau"
    | "Pacific/Pitcairn"
    | "Pacific/Pohnpei"
    | "Pacific/Ponape"
    | "Pacific/Port_Moresby"
    | "Pacific/Rarotonga"
    | "Pacific/Saipan"
    | "Pacific/Samoa"
    | "Pacific/Tahiti"
    | "Pacific/Tarawa"
    | "Pacific/Tongatapu"
    | "Pacific/Truk"
    | "Pacific/Wake"
    | "Pacific/Wallis"
    | "Pacific/Yap"
    | "Poland"
    | "Portugal"
    | "ROC"
    | "ROK"
    | "Singapore"
    | "Turkey"
    | "UCT"
    | "US/Alaska"
    | "US/Aleutian"
    | "US/Arizona"
    | "US/Central"
    | "US/East-Indiana"
    | "US/Eastern"
    | "US/Hawaii"
    | "US/Indiana-Starke"
    | "US/Michigan"
    | "US/Mountain"
    | "US/Pacific"
    | "US/Samoa"
    | "UTC"
    | "Universal"
    | "W-SU"
    | "WET"
    | "Zulu"
