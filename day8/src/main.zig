const std = @import("std");
const ArrayList = std.ArrayList;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const stdin = std.io.getStdIn();

    var list = ArrayList(ArrayList(u8)).init(allocator);
    defer list.deinit();

    var cols: usize = 0;

    var line: []u8 = try stdin.readToEndAlloc(allocator, 1024 * 20);
    defer allocator.free(line);

    var iter_line = std.mem.split(u8, line, "\n");
    while (iter_line.next()) |l| {
        var row = ArrayList(u8).init(allocator);

        for (l) |c| {
            const digit: u8 = c - '0';
            try row.append(digit);
        }

        try list.append(row);
        row.clearRetainingCapacity();
        cols += 1;
    }

    const rows: usize = list.items[0].items.len;

    var count = part1(list, rows, cols);

    std.debug.print("result: {}\n", .{count});
}

fn part1(list: ArrayList(ArrayList(u8)), rows: usize, cols: usize) usize {
    var count: usize = cols * 2 + (rows - 2) * 2;

    for (1..rows - 1) |i| {
        for (1..cols - 1) |j| {
            var current_tree = list.items[i].items[j];
            var up_visible = false;
            var down_visible = false;
            var left_visible = false;
            var right_visible = false;

            std.debug.print("current_tree: {d}\n", .{current_tree});

            // upwards
            var up: i8 = @as(i8, @intCast(i)) - 1;
            while (up >= 0) {
                if (list.items[@as(usize, @intCast(up))].items[j] >= current_tree) {
                    up_visible = false;
                    break;
                } else {
                    up_visible = true;
                }

                up -= 1;
            }

            // downwards
            var down = i + 1;
            while (down < rows) : (down += 1) {
                if (list.items[down].items[j] >= current_tree) {
                    down_visible = false;
                    break;
                } else {
                    down_visible = true;
                }
            }

            // leftwards
            var left = @as(i8, @intCast(j)) - 1;
            while (left >= 0) {
                if (list.items[i].items[@as(usize, @intCast(left))] >= current_tree) {
                    left_visible = false;
                    break;
                } else {
                    left_visible = true;
                }
                left -= 1;
            }

            // rightwards
            var right = j + 1;
            while (right < cols) {
                if (list.items[i].items[right] >= current_tree) {
                    right_visible = false;
                    break;
                } else {
                    right_visible = true;
                }

                right += 1;
            }

            if (up_visible or down_visible or left_visible or right_visible) {
                count += 1;
            }
        }
    }
    return count;
}

test "test digits" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!

    const input = "45142";
    for (input) |c| {
        const digit = c - '0';
        try list.append(digit);
    }

    try std.testing.expectEqualSlices(i32, &[_]i32{ 4, 5, 1, 4, 2 }, list.items);
}
