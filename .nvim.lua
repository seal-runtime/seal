-- Per-project luwu-lsp settings for seal projects. Copy this into a project root as .nvim.lua (nvim loads it
-- through exrc). The Luwu FFlags and the server path are global, in lua/plugins/luau_lsp.lua.
require("luau-lsp").setup {
  platform = {
    type = "standard",
  },
  types = {
    definition_files = {
      ["@seal"] = "./.seal/typedefs/globals.d.luau",
    },
    documentation_files = { "./.seal/typedefs/docs.json" },
  },
}
