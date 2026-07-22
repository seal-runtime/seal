local fs = require("@std/fs")

local parser = require("@libraries/LuauParser/Parser")
local path = ""
const function update_path(p: string)
    path = string.gsub(p, fs.path.cwd(), ".")
end

export class BlockComment
    public start: vector
    public finish: vector

    function new(begin_col: number, begin_line: number, end_col: number, end_line: number)
        return BlockComment {
            start = vector.create(begin_col + 1, begin_line),
            finish = vector.create(end_col + 1, end_line)
        }
    end
    function __tostring(self)
        return `BlockComment<({path}:{self.start.y}:{self.start.x}), ({path}:{self.finish.y}:{self.finish.x})>`
    end
    function extract_from_lines(
        self,
        lines: { string },
        bytes_by_line: { number },
        content: string
    ): (string, number)
        const start_column = self.start.x
        const start_line = self.start.y

        const end_column = self.finish.x
        const end_line = self.finish.y

        local start_pos: number
        local end_pos: number

        for ln, line in lines do
            if ln == start_line then
                start_pos = bytes_by_line[ln] + start_column
            end
            if ln == end_line then
                end_pos = bytes_by_line[ln] + end_column
            end
        end

        return string.sub(content, start_pos, end_pos), end_pos
    end
end

export class LineComment
    public line: number
    public first_column: number

    function new(line: number, col: number)
        return LineComment {
            line = line + 1,
            first_column = col + 1
        }
    end
    function __tostring(self)
        return `LineComment<line: {self.line}, column: {self.first_column}>`
    end
    function extract_from_lines(
        self,
        lines: { string },
        bytes_by_line: { number },
        content: string
    ): (string, number)
        local line = lines[self.line]
        assert(line ~= nil, `LineComment line at {self.line} is nil in lines array`)

        local end_byte = bytes_by_line[self.line]
        assert(end_byte ~= nil, `we don't have byte count for line {self.line}`)
        
        return string.sub(line, self.first_column), end_byte
    end
end

export function build(
    parsed: parser.Result,
    lines: { string },
    path_for_update: string,
    content: string
): { BlockComment | LineComment }
    update_path(path_for_update)

    const comments: { BlockComment | LineComment } = {}
    for _, c in parsed.commentLocations do
        if c.type == 283 then -- Block comment
            table.insert(comments, BlockComment.new(
                c.location.begin.column,
                c.location.begin.line,
                c.location.end_.column,
                c.location.end_.line
            ))
        elseif c.type == 282 then
            table.insert(comments, LineComment.new(
                c.location.begin.line,
                c.location.begin.column
            ))
        end
    end

    return comments
end