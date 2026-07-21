local _types = require("@self/_types")
local ar = require("@self/ar")
local deb = require("@self/deb")
local sevenz = require("@self/sevenz")
local tar = require("@self/tar")
local zip = require("@self/zip")

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat
export type ContainerFormat = _types.ContainerFormat
export type SingleFileFormat = _types.SingleFileFormat

export type archive = {
    zip: zip.zip,
    tar: tar.tar,
    ar: ar.ar,
    deb: deb.deb,
    sevenz: sevenz.sevenz,
    create: () -> Archive,
    load: (bytes: buffer, format: ArchiveFormat, options: ArchiveOptions?) -> Archive,
}

return {} :: archive