---@alias OS
---| "linux"
---| "windows"
---| "macos"
---| "android"
---| "ios"
---| "openbsd"
---| "freebsd"
---| "netbsd"
---| "wasi"
---| "aix"
---| "apple"
---| "dragonfly"
---| "emscripten"
---| "espidf"
---| "fortanix"
---| "uefi"
---| "fuchsia"
---| "haiku"
---| "hermit"
---| "watchos"
---| "visionos"
---| "tvos"
---| "horizon"
---| "hurd"
---| "illumos"
---| "l4re"
---| "nto"
---| "redox"
---| "solaris"
---| "solid_asp3"
---| "vexos"
---| "vita"
---| "vxworks"
---| "xous"

---@class WhichOS
---@field get_os fun(): OS
---@field is_windows fun(): boolean
---@field is_macos fun(): boolean
---@field is_linux fun(): boolean

---@type WhichOS
local M

local ok, native = pcall(require, "which_os")

if not ok then
  error("which_os native module failed to load")
end

M = native

return M
