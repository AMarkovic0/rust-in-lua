ffi = require("ffi")

function readHeader(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end

local headerName = "rua.h"
ffi.cdef(readHeader(headerName))

local rua = ffi.load("./target/debug/librua.so")

local a = 1
local b = 2

print("Call rust add function: ", rua.rust_add(a, b))
