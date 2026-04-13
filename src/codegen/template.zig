const std = @import("std");
extern fn vm_entry(ptr: [*]const u8, len: usize) void;
var program = @embedFile("path/to/bytcode");

pub fn main() void {
    vm_entry(program.ptr, program.len);
}
