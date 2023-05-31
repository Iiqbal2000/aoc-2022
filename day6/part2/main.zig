const std = @import("std");
const testing = std.testing;

pub fn main() !void {
    const in = std.io.getStdIn();
    var buf = std.io.bufferedReader(in.reader());

   var r = buf.reader();

   var msg_buf: [4096]u8 = undefined;
   const res = try r.readUntilDelimiterOrEof(&msg_buf, '\n');

   if (res) |_| {
    var result: u16 = get_first_marker(msg_buf[0..]);
    std.debug.print("result: {d}\n", .{result});
   }
}

fn get_first_marker(datastream: []const u8) u16 {
    const slide_size = 14;

    for (datastream, 0..) |_, left| {
        if (left+slide_size <= datastream.len) {
                        
            var uniqness: bool = true;

            var buffer: []const u8 = datastream[left..left+slide_size];
            
            outer: for (buffer, 0..) |b1, bufferI| {
                for (buffer, (bufferI+1)..) |_, b2Index| {
                    if (b2Index < buffer.len) {
                        if (b1 == buffer[b2Index]) {
                            uniqness = false;
                            break :outer;
                        }
                        
                    }

                }
            }

            if (uniqness) {
                std.debug.print("\n{s} {d}:{d}\n", .{datastream[left..left+slide_size], left, left+slide_size});
                return @intCast(u16, left+slide_size);
            }
        }
    }
    return 0;
}

test "example cases" {
    try testing.expect(get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == @as(u16, 19));

    try testing.expect(get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz") == @as(u16, 23));

    try testing.expect(get_first_marker("nppdvjthqldpwncqszvftbrmjlhg") == @as(u16, 23));

    try testing.expect(get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == @as(u16, 29));

    try testing.expect(get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == @as(u16, 26));

}
