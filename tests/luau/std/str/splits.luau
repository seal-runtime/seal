local str = require("@std/str")

local function checklines(label: string, s: string, trim: boolean?, include_terminator: boolean?, expected: { string })
    local result = str.splitlines(s, trim, include_terminator)
    local ok = #result == #expected
    if ok then
        for i = 1, #expected do
            if result[i] ~= expected[i] then
                ok = false
                break
            end
        end
    end
    if not ok then
        print(`FAIL: {label}`)
        print("  expected:", expected)
        print("  got:", result)
        error(`str.splitlines test failed: {label}`)
    end
end

local function basicsplitlines()
    checklines("basic \\n", "a\nb\nc", nil, nil, { "a", "b", "c" })
    checklines("basic \\r\\n", "a\r\nb\r\nc", nil, nil, { "a", "b", "c" })
    checklines("mixed \\n and \\r\\n", "a\r\nb\nc", nil, nil, { "a", "b", "c" })
    checklines("no newline at all", "hello", nil, nil, { "hello" })
end

basicsplitlines()

-- empty lines between/before other lines are always preserved, regardless of include_terminator
local function emptylinespreserved()
    checklines("leading empty line", "\na\nb", nil, nil, { "", "a", "b" })
    checklines("empty line between others", "a\n\nb", nil, nil, { "a", "", "b" })
    checklines("crlf empty line between others", "a\r\n\r\nb", nil, nil, { "a", "", "b" })
    checklines("multiple consecutive empties", "a\nb\n\n\nc", nil, nil, { "a", "b", "", "", "c" })
end

emptylinespreserved()

-- by default, a trailing line terminator with nothing after it doesn't manufacture a phantom
-- trailing "" - but a "real" empty line closed by an earlier terminator is still kept
local function trailingterminatorexcludedbydefault()
    checklines("simple trailing newline", "a\nb\n", nil, nil, { "a", "b" })
    checklines("trailing newline keeps an earlier empty line", "cats\n\n", nil, nil, { "cats", "" })
    checklines("just a single newline", "\n", nil, nil, { "" }) -- the \n closes an empty line; no further tail
    checklines("empty string", "", nil, nil, {})
end

trailingterminatorexcludedbydefault()

-- include_terminator=true forces that trailing "" back in, independent of trim_trailing_whitespace
local function includeterminator()
    checklines("trailing newline + include_terminator, trim default", "a\nb\n", nil, true, { "a", "b", "" })
    checklines("trailing newline + include_terminator, trim=false", "a\nb\n", false, true, { "a", "b", "" })
    checklines("single newline + include_terminator", "\n", nil, true, { "", "" })
    checklines("trailing double newline + include_terminator", "cats\n\n", nil, true, { "cats", "", "" })
end

includeterminator()

-- a genuinely non-empty (whitespace-only) unterminated tail is always kept regardless of
-- include_terminator; trim_trailing_whitespace controls whether its content becomes ""
local function whitespaceonlytail()
    checklines("whitespace-only tail, trimmed", "cats\n   ", nil, false, { "cats", "" })
    checklines("whitespace-only tail, trimmed, include_terminator=true (same)", "cats\n   ", nil, true, { "cats", "" })
    checklines("whitespace-only tail, untrimmed", "cats\n   ", false, false, { "cats", "   " })
    checklines("whitespace-only tail, untrimmed, include_terminator=true (same)", "cats\n   ", false, true, { "cats", "   " })
end

whitespaceonlytail()

-- non-utf-8 input falls back to a different code path internally; make sure it respects
-- include_terminator the same way the utf-8 fast path does
local function nonutf8fallback()
    local s = "a\255b\n"
    assert(str.encoding(s) == "Binary", "sanity check: this string should not be valid utf-8")
    checklines("non-utf8 default", s, nil, nil, { "a\255b" })
    checklines("non-utf8 + include_terminator", s, nil, true, { "a\255b", "" })
end

nonutf8fallback()

-- real-world multiline content ending in a trailing newline
local daleki_obriyi_lyrics = [[
    "Далекі Обрії" by Ignea

    Неначе у золотій клітці
    Охоплена гамою барв
    Не cтерплять ми бідні зіниці
    Щодня - небуденний чар
    Міцні татуйовані лиця
    Боронять минуле своє
    Прощальне слово "верніться"
    Спіткає усюди мене
    Там де жодних листів не буває
    Скільки років кому не питають
    Полюю на кадри світу, що зникає
    Далекі обрії кличуть і манять
    Тремтіла, ламалась, благала
    Звивалась фільмова тасьма
    Дно пропасті - карна виправа
    Розсудить пралісна імла
    Коралеві скелі і мури
    Квітковий вівтар на ножах
    Тепер мене звати Теура
    Тепер я - багряний птах!
    Мої світлини не в змозі передати
    Неземної краси і омани
    Люди там не живуть - виживають
    Далекі обрії ваблять, жахають
]]

local function lyricsendinwithtrailingnewline()
    local without_terminator = str.splitlines(daleki_obriyi_lyrics, false, false)
    local with_terminator = str.splitlines(daleki_obriyi_lyrics, false, true)
    assert(#with_terminator == #without_terminator + 1, "include_terminator should add exactly one trailing element")
    assert(with_terminator[#with_terminator] == "", "the extra trailing element should be an empty string")
    for i = 1, #without_terminator do
        assert(with_terminator[i] == without_terminator[i], `line {i} should be unaffected by include_terminator`)
    end
end

lyricsendinwithtrailingnewline()
