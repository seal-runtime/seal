local str = require("@std/str")

local function basic_detection()
    assert(str.encoding("hello world") == "Utf8", "plain ascii should detect as Utf8")
    assert(str.encoding("héllo wörld 🦭") == "Utf8", "multibyte utf-8 without a bom should still detect as Utf8")
    assert(str.encoding("\239\187\191hello") == "Utf8Bom", "utf-8 bom should detect as Utf8Bom")
    assert(str.encoding("\254\255\0h\0i") == "Utf16BEBom", "utf-16 be bom should detect as Utf16BEBom")
    assert(str.encoding("\255\254h\0i\0") == "Utf16LEBom", "utf-16 le bom should detect as Utf16LEBom")

    -- bare (bom-less) ascii utf-16 is indistinguishable from valid utf-8 (nul is a legal codepoint),
    -- so the null-byte-density heuristic only ever gets a chance to kick in for non-ascii text,
    -- where a lone high byte like é's 0xE9 breaks utf-8 validity
    local non_ascii_sample = "café wörld"
    local bare_utf16be = string.sub(str.convert(non_ascii_sample, "Utf16BEBom"), 3)
    local bare_utf16le = string.sub(str.convert(non_ascii_sample, "Utf16LEBom"), 3)
    assert(str.encoding(bare_utf16be) == "Utf16BE", "bare (bom-less) utf-16be should be detected via null byte density heuristic")
    assert(str.encoding(bare_utf16le) == "Utf16LE", "bare (bom-less) utf-16le should be detected via null byte density heuristic")

    assert(str.encoding("\1\2\3\4\255\254\253\0\0\1") == "Binary", "random non-utf8 bytes without a bom or utf-16 signature should be Binary")
end
basic_detection()

local function roundtrip()
    local original = "seals 🦭 are cute, née cool café"
    for _, encoding in {"Utf8", "Utf8Bom", "Utf16LE", "Utf16LEBom", "Utf16BE", "Utf16BEBom"} do
        local converted = str.convert(original, encoding :: any)
        assert(str.encoding(converted) == encoding, `convert to {encoding} should itself be detected as {encoding}`)
        local back = str.convert(converted, "Utf8")
        assert(back == original, `roundtripping through {encoding} should preserve the original string`)
    end
end
roundtrip()

local function explicit_from()
    local original = "plain ascii, no funny business"
    local as_utf16be = str.convert(original, "Utf16BE")
    -- str.convert would autodetect this fine without `from`, but passing it explicitly should also work
    local back = str.convert(as_utf16be, "Utf8", "Utf16BE")
    assert(back == original, "explicit `from` should decode correctly")
end
explicit_from()

local function buffer_input()
    local as_buffer = buffer.fromstring("plain text as a buffer")
    assert(str.encoding(as_buffer) == "Utf8", "buffer input should be detected the same as string input")
    local converted = str.convert(as_buffer, "Utf16LEBom")
    assert(str.encoding(converted) == "Utf16LEBom", "buffer input should convert correctly")
end
buffer_input()

local function convert_to_binary_is_a_passthrough()
    -- "Binary" isn't a real text encoding, so converting to it should just hand back the exact same bytes untouched,
    -- even for bytes that aren't valid text in any encoding
    local garbage = "\1\2\3\4\255\254\253\0\0\1"
    assert(str.convert(garbage, "Binary") == garbage, "converting to Binary should return the original bytes unchanged")

    local utf16 = str.convert("seals 🦭", "Utf16LEBom")
    assert(str.convert(utf16, "Binary") == utf16, "converting to Binary should pass through regardless of the source encoding")
end
convert_to_binary_is_a_passthrough()

-- one of the more cursed exports excel will hand you: "Unicode Text (*.txt)" save-as, renamed to .csv.
-- it's tab-delimited (not comma!) despite masquerading as a csv, utf-16le with a bom, uses \r\n line endings,
-- and still escapes embedded quotes by doubling them like a normal csv would
local excel_csv_utf16lebom =
    "\255\254\78\0\97\0\109\0\101\0\9\0\78\0\111\0\116\0\101\0\115\0\9\0\65\0\109\0\111\0\117\0\110\0\116\0\13\0\10\0\67\0\97\0\102\0\233\0\32\0\21\38\9\0\83\0\97\0\105\0\100\0\32\0\34\0\34\0\104\0\101\0\108\0\108\0\111\0\44\0\32\0\119\0\111\0\114\0\108\0\100\0\33\0\34\0\34\0\9\0\49\0\44\0\50\0\51\0\52\0\46\0\53\0\48\0\13\0\10\0\82\0\233\0\115\0\117\0\109\0\233\0\32\0\62\216\173\221\9\0\69\0\109\0\111\0\106\0\105\0\32\0\116\0\101\0\115\0\116\0\32\0\60\216\137\223\61\216\37\221\9\0\45\0\52\0\50\0\13\0\10\0"

local function excel_export()
    assert(str.encoding(excel_csv_utf16lebom) == "Utf16LEBom", "excel's cursed unicode text export should be detected as Utf16LEBom")

    local nice_utf8 = str.convert(excel_csv_utf16lebom, "Utf8")
    assert(str.encoding(nice_utf8) == "Utf8", "converted csv should no longer look like utf-16")

    local expected =
        "Name\tNotes\tAmount\r\n"
        .. "Café ☕\tSaid \"\"hello, world!\"\"\t1,234.50\r\n"
        .. "Résumé 🦭\tEmoji test 🎉🔥\t-42\r\n"

    assert(nice_utf8 == expected, "converted csv contents should exactly match the original excel export")
end
excel_export()