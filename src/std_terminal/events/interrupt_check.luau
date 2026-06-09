local err = require("@std/err")
export type KeyModifiers = {
    ctrl: boolean,
    shift: boolean,
    alt: boolean,
}

export type KeyEvent = {
    type: "Key",
    key: string,
    kind: string,
    modifiers: KeyModifiers,
}
export type MouseEvent = {
    type: "Mouse",
    kind: string,
    position: vector,
    modifiers: KeyModifiers,
}
export type ResizeEvent = {
    type: "Resize",
    size: vector,
}
export type FocusGained = {
    type: "FocusGained",
}
export type FocusLost = {
    type: "FocusLost",
}
export type PasteEvent = {
    type: "Paste",
    contents: string,
}
export type Empty = {
    type: "Empty",
}

export type TerminalEvent = 
    | KeyEvent 
    | MouseEvent
    | ResizeEvent
    | PasteEvent
    | FocusGained 
    | FocusLost 
    | Empty

--- returns `true` if event is a SIGINT, false if it's an EOF, nil if it's neither, and an error
--- if the table is of the wrong type
local function interrupt_check(event: TerminalEvent): err.Result<boolean?>
    if typeof(event) ~= "table" then
        return err.message(`event is not a TerminalEvent; expected table, got {typeof(event)}`)
    end

    if event.type ~= "Key" then
        return nil
    end

    if event.modifiers and event.modifiers.ctrl then
        if event.key and event.key == "c" then
            return true
        elseif event.key and event.key == "d" then
            return false
        end
    end

    return nil
end

return interrupt_check