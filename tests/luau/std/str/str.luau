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