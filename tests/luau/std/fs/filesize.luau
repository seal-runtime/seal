-- tests for our custom FileSize extern type userdata

local filesize = require("@std/fs/filesize")

local function lt()
    local five_bytes = filesize.bytes(5)
    local six_bytes = filesize.bytes(6)
    assert(five_bytes < six_bytes, "FileSize < not work")
    assert(six_bytes > five_bytes, "FileSize > not work")
end

lt()

local function le()
    local five_bytes = filesize.bytes(5)
    local also_five_bytes = filesize.bytes(5)
    local six_bytes = filesize.bytes(6)
    assert(five_bytes <= also_five_bytes, "FileSize <= should hold for equal sizes")
    assert(five_bytes <= six_bytes, "FileSize <= should hold when less than")
end

le()

local function eq()
    local a = filesize.bytes(11)
    local b = filesize.bytes(11)
    assert(a == b, "equal FileSizes should be ==")
    local c = filesize.bytes(12)
    assert(a ~= c, "different FileSizes should not be ==")
end

eq()

local function add()
    local five = filesize.bytes(5)
    local six = filesize.bytes(6)
    assert(five + six == filesize.bytes(11), "FileSize add incorrect")
end

add()

local function sub()
    local eleven = filesize.bytes(11)
    local six = filesize.bytes(6)
    assert(eleven - six == filesize.bytes(5), "FileSize sub incorrect")
end

sub()

local function mul()
    local five = filesize.bytes(5)
    assert(five * filesize.bytes(6) == filesize.bytes(30), "FileSize * FileSize incorrect")
    assert(five * 3 == filesize.bytes(15), "FileSize * integer incorrect")
    assert(filesize.bytes(10) * 1.5 == filesize.bytes(15), "FileSize * float incorrect")
end

mul()

local function div()
    local thirty = filesize.bytes(30)
    assert(thirty / filesize.bytes(6) == filesize.bytes(5), "FileSize / FileSize incorrect")
    assert(thirty / 3 == filesize.bytes(10), "FileSize / integer incorrect")
    assert(filesize.bytes(15) / 1.5 == filesize.bytes(10), "FileSize / float incorrect")
end

div()

local function mul_div_errors()
    local five = filesize.bytes(5)
    local s1, res1 = pcall(function()
        return five * -2
    end)
    assert(s1 == false, "multiplying by a negative number should error")
    assert(tostring(res1):match("must be positive"), "negative multiply error message incorrect")

    local s2, res2 = pcall(function()
        return five / 0
    end)
    assert(s2 == false, "dividing by zero should error")
    assert(tostring(res2):match("divide by zero"), "divide by zero error message incorrect")

    local s3, res3 = pcall(function()
        return five / filesize.bytes(0)
    end)
    assert(s3 == false, "dividing by a zero-byte FileSize should error")
    assert(tostring(res3):match("zero%-byte FileSize"), "divide by zero-byte FileSize error message incorrect")
end

mul_div_errors()

local function constructors()
    assert(filesize.kilobytes(1) == filesize.bytes(1_024), "1 KB should be 1024 bytes")
    assert(filesize.megabytes(1) == filesize.kilobytes(1_024), "1 MB should be 1024 KB")
    assert(filesize.gigabytes(1) == filesize.megabytes(1_024), "1 GB should be 1024 MB")
    assert(filesize.terabytes(1) == filesize.gigabytes(1_024), "1 TB should be 1024 GB")
end

constructors()

local function fractional_constructors()
    -- 1.5 KB = 1536 bytes
    assert(filesize.kilobytes(1.5) == filesize.bytes(1_536), "1.5 KB should be 1536 bytes")
    -- 1.5 MB = 1536 KB
    assert(filesize.megabytes(1.5) == filesize.kilobytes(1_536), "1.5 MB should be 1536 KB")
end

fractional_constructors()

local function as_units()
    local one_mb = filesize.megabytes(1)
    assert(one_mb:as_bytes() == 1_048_576, "as_bytes should return raw byte count")
    assert(one_mb:as_kilobytes() == 1_024, "as_kilobytes should return 1024 for 1 MB")
    assert(one_mb:as_megabytes() == 1, "as_megabytes should return 1 for 1 MB")
    assert(one_mb:as_gigabytes() == 1 / 1_024, "as_gigabytes should return fractional GB")
end

as_units()

local function format_method()
    local one_mb = filesize.megabytes(1)
    assert(one_mb:format("Bytes") == "1048576 B", `format Bytes got {one_mb:format("Bytes")}`)
    assert(one_mb:format("KB") == "1024.00 KB", `format KB got {one_mb:format("KB")}`)
    assert(one_mb:format("MB") == "1.00 MB", `format MB got {one_mb:format("MB")}`)
end

format_method()

local function display_and_tostring()
    assert(filesize.bytes(512):display() == "512 bytes", `display bytes got {filesize.bytes(512):display()}`)
    assert(filesize.kilobytes(2):display() == "2.00 KB", `display KB got {filesize.kilobytes(2):display()}`)
    assert(filesize.megabytes(3):display() == "3.00 MB", `display MB got {filesize.megabytes(3):display()}`)
    assert(filesize.gigabytes(4):display() == "4.00 GB", `display GB got {filesize.gigabytes(4):display()}`)
end

display_and_tostring()

local function underflow_errors()
    local twenty_gigs = filesize.gigabytes(20)
    local smol = filesize.bytes(1_025)
    local s, res = pcall(function()
        return smol - twenty_gigs
    end)
    assert(s == false, "underflowing should result in error")
    assert(typeof(res) == "error", "underflow error when subtracting FileSizes should be made of error")
    assert(tostring(res):match("subtraction would underflow"), "FileSize subtraction error message should say that result would underflow")
end

underflow_errors()
