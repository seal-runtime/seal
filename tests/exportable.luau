export class MidnightFrost
    public cool: boolean
    function new()
        return MidnightFrost { cool = true }
    end
    function __tostring(self)
        return `MidnightFrost is {if self.cool then "cool" else "eh"}`
    end
end
