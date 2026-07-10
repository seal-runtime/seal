local str = require("@std/str")
local colors = require("@std/io/colors")

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

local function basichelperfunctions()
    assert(
        str.startswith(
            str.slice(
                str.trimfront(daleki_obriyi_lyrics), 0, 12
            ),
            "\"Дал"
        ) == true, "str oops"
    )
end

basichelperfunctions()

local function strsplit()
    local a1 = "Далекі"
    assert(str.split(a1)[3] == "л", "Далекі third index not л")
    
    local chars = str.split("seals 🦭 say hi", " ", "hi")
    assert(
        chars[1] == "seals" 
        and chars[2] == "🦭"
        and chars[3] == "say"
        and chars[4] :: any == nil, -- we had an issue with empty final string from separators
        "str.split with seal emoji not worked?"
    )
end

strsplit()

local function strsplitbutoopswehaveinvalidunicode()
    local fs = require("@std/fs")
    local nanuk_png_path = fs.path.join(".", "tests", "data", "nanuk.png")
    local png_header_buffy = fs.readbytes(nanuk_png_path, 0, 124)
    local hstring = buffer.tostring(png_header_buffy)
    local _ = str.split(hstring) -- atp we want to just make sure it doesn't fail
end

strsplitbutoopswehaveinvalidunicode()

local function strgraphemes()
    local utf8_string = "हाय मैं एक सील हूँ 🦭 (hi i am a seal 🦭)"
    -- note we have some weirdness with iterating over hindi characters w/ unicode_reader's 
    -- extended mode which i assume is the default from it using unicode_segmentation
    -- this means we can get graphemes that dont render properly in terminals
    for offset, grapheme in str.graphemes(utf8_string) do
        if offset == 7 then
            assert(grapheme == "क", "7th byte not क")
        end
    end
end

strgraphemes()

local function splitaround()
    local line = `function Cat.meow(name: string, age: number)`
    local tokens = str.splitaround(line, " ", ".", "(", ":", ",", ")")
    assert(tokens[1] == "function", "first token should be function")
    assert(tokens[3] == "Cat", "third token should be Cat")
    assert(tokens[9] == " ", "9th token should be whitespace ' '")
    assert(tokens[#tokens] == ")", "last token should be )")
end

splitaround()

local function splitaroundunicode()
    local keepy = "हाय मैं एक सील हूँ 🦭 (hi i am a seal 🦭)"
    local sealed = str.splitaround(keepy, "🦭")
    assert(sealed[2] == "🦭", "2nd index should be 🦭")
end

splitaroundunicode()

local function assertarray(got: { string }, expected: { string })
    for index, value in expected do
        assert(got[index] == value, `incorrect got[{index}]; expected '{value}', got: '{got[index]}'`)
    end
end

local function splitbefore()
    local text = "we, are, the, dead,"
    local splitted = str.splitbefore(text, ", ", ",")
    assertarray(splitted, {"we", ", are", ", the", ", dead", ","})
end

splitbefore()

local function splitbeforemessages()
    local messages = "[INFO] message\nnext line of message\n[WARN] bad warning\n[ERROR] message\n stack traceback"
    local splitted = str.splitbefore(messages, "[INFO]", "[WARN]", "[ERROR]")
    assertarray(splitted, {"[INFO] message\nnext line of message\n", "[WARN] bad warning\n", "[ERROR] message\n stack traceback"})
end

splitbeforemessages()

local function splitafter()
    local text = "we, are, the, dead"
    local splitted = str.splitafter(text, ", ")
    assertarray(splitted, {"we, ", "are, ", "the, ", "dead"})
end

splitafter()

local function splitafterheaders()
    local httpheaders = "Content-Type: text/html\r\nContent-Length: 123\r\nConnection: keep-alive\r\n"
    local splitted = str.splitafter(httpheaders, "\r\n")
    assertarray(splitted, {"Content-Type: text/html\r\n", "Content-Length: 123\r\n", "Connection: keep-alive\r\n"})
end

splitafterheaders()

--- str.width should strip ansi color/style codes before measuring, since TUI code needs
--- accurate display width for colorized text (see std_io/prompt/prompt.luau and examples/double_column_picker.luau)
local function widthignorescolors()
    local plain = "Hello"
    assert(str.width(plain) == 5, "plain width should be 5")

    assert(str.width(colors.red(plain)) == 5, "red should not affect width")
    assert(str.width(colors.bold.blue(plain)) == 5, "bold+blue should not affect width")
    assert(str.width(colors.style.bold(plain)) == 5, "bold style should not affect width")
    assert(str.width(colors.style.dim(plain)) == 5, "dim style should not affect width")
    assert(str.width(colors.style.underline(plain)) == 5, "underline style should not affect width")
    assert(str.width(colors.rgb(vector.create(255, 100, 50), plain)) == 5, "truecolor rgb should not affect width")

    local nested = colors.style.bold(colors.style.underline(colors.red(plain)))
    assert(str.width(nested) == 5, "nested styles should not affect width")

    local bg_and_reset = colors.codes.RED_BG .. plain .. colors.codes.RESET
    assert(str.width(bg_and_reset) == 5, "raw background code + reset should not affect width")
end

widthignorescolors()

local function widthhandlesnonasciiwithcolors()
    local wide = colors.bold.green("田中良")
    assert(str.width(wide) == 6, `expected width 6 (2 per cjk char), got {str.width(wide)}`)
end

widthhandlesnonasciiwithcolors()
local function reverse()
    assert(str.reverse("hello") == "olleh")
    assert(str.reverse("") == "")
    assert(str.reverse("a") == "a")
    assert(str.reverse("seals 🦭 rock") == "kcor 🦭 slaes", str.reverse("seals 🦭 rock"))
    assert(str.reverse("田中良") == "良中田", str.reverse("田中良"))

    -- non-utf8 input shouldn't crash, even though byte-reversal isn't meaningful text
    local nonutf8 = "a\255b"
    assert(str.encoding(nonutf8) == "Binary")
    assert(#str.reverse(nonutf8) == 3)
end

reverse()

local function rightpad()
    assert(str.rightpad("hi", 5) == "hi   ")
    assert(str.rightpad("hi", 5, "-") == "hi---")
    assert(str.rightpad("hello", 3) == "hello") -- no padding needed
    assert(str.leftpad("hi", 5) == "   hi") -- sanity check leftpad still works
end

rightpad()

local function replacebasic()
    assert(str.replace("the cat sat on the mat", "at", "og") == "the cog sog on the mog")
    assert(str.replace("aaaa", "a", "b") == "bbbb")
    assert(str.replace("hello", "xyz", "abc") == "hello") -- no match
    assert(str.replace("hello", "hello", "") == "") -- replace with empty
end

replacebasic()

local function replacemaxreplacements()
    assert(str.replace("aaaa", "a", "b", 2) == "bbaa")
    assert(str.replace("aaaa", "a", "b", 0) == "aaaa")
    assert(str.replace("aaaa", "a", "b", 100) == "bbbb") -- more than available matches
end

replacemaxreplacements()

local function replacewithindices()
    -- string.sub-style indexing: only occurrences within [start_index, end_index] are replaced
    assert(str.replace("aaaa", "a", "b", nil, 2) == "abbb", str.replace("aaaa", "a", "b", nil, 2))
    assert(str.replace("aaaa", "a", "b", nil, 1, 2) == "bbaa", str.replace("aaaa", "a", "b", nil, 1, 2))
    assert(str.replace("aaaa", "a", "b", nil, -2) == "aabb", str.replace("aaaa", "a", "b", nil, -2))
    assert(str.replace("aaaa", "a", "b", nil, -2, -1) == "aabb")

    -- max_replacements and an index range together
    assert(str.replace("aaaaaa", "a", "b", 2, 3) == "aabbaa", str.replace("aaaaaa", "a", "b", 2, 3))
end

replacewithindices()

local function replaceerrorsonemptyold()
    local ok = pcall(str.replace, "hello", "", "x")
    assert(not ok, "str.replace should error when old is an empty string")
end

replaceerrorsonemptyold()

local function splitfront()
    assert(str.splitfront("a,b,c,d", ",")[1] == "a")
    assert(str.splitfront("a,b,c,d", ",")[2] == "b,c,d")
    assert(#str.splitfront("a,b,c,d", ",") == 2)

    local two = str.splitfront("a,b,c,d", ",", 2)
    assert(#two == 3 and two[1] == "a" and two[2] == "b" and two[3] == "c,d")

    -- max larger than available separators behaves like a full split
    local full = str.splitfront("a,b,c,d", ",", 100)
    assert(#full == 4 and full[1] == "a" and full[4] == "d")

    -- max=0 means no splitting at all
    local none = str.splitfront("a,b,c,d", ",", 0)
    assert(#none == 1 and none[1] == "a,b,c,d")

    -- no separator found
    local nomatch = str.splitfront("hello", ",")
    assert(#nomatch == 1 and nomatch[1] == "hello")
end

splitfront()

local function splitback()
    assert(str.splitback("a,b,c,d", ",")[1] == "a,b,c")
    assert(str.splitback("a,b,c,d", ",")[2] == "d")
    assert(#str.splitback("a,b,c,d", ",") == 2)

    local two = str.splitback("a,b,c,d", ",", 2)
    assert(#two == 3 and two[1] == "a,b" and two[2] == "c" and two[3] == "d")

    local full = str.splitback("a,b,c,d", ",", 100)
    assert(#full == 4 and full[1] == "a" and full[4] == "d")

    local none = str.splitback("a,b,c,d", ",", 0)
    assert(#none == 1 and none[1] == "a,b,c,d")

    local nomatch = str.splitback("hello", ",")
    assert(#nomatch == 1 and nomatch[1] == "hello")
end

splitback()

local function splitfrontbackerrorsonemptysep()
    assert(not pcall(str.splitfront, "hello", ""), "str.splitfront should error on empty sep")
    assert(not pcall(str.splitback, "hello", ""), "str.splitback should error on empty sep")
end

splitfrontbackerrorsonemptysep()

local function splitfrontbackworkwithanysepwidth()
    -- multi-character separator
    local a = str.splitfront("a::b::c::d", "::")
    assert(#a == 2 and a[1] == "a" and a[2] == "b::c::d")

    local b = str.splitback("a::b::c::d", "::", 2)
    assert(#b == 3 and b[1] == "a::b" and b[2] == "c" and b[3] == "d")

    -- word-length separator
    local c = str.splitfront("one--sep--two--sep--three", "--sep--")
    assert(#c == 2 and c[1] == "one" and c[2] == "two--sep--three")

    -- crlf (2-byte) separator
    local d = str.splitback("line1\r\nline2\r\nline3", "\r\n")
    assert(#d == 2 and d[1] == "line1\r\nline2" and d[2] == "line3")

    -- long (20-byte) separator
    local long_sep = string.rep("=", 20)
    local e_str = "start" .. long_sep .. "middle" .. long_sep .. "end"
    local e = str.splitfront(e_str, long_sep)
    assert(#e == 2 and e[1] == "start" and e[2] == "middle" .. long_sep .. "end")

    -- non-overlapping match semantics with a repeating-character separator
    local f = str.splitfront("aaaa", "aa")
    assert(#f == 1 and f[1] == "aa")
    local g = str.splitfront("aaaa", "aa", 2)
    assert(#g == 0)
end

splitfrontbackworkwithanysepwidth()
