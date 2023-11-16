enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "⬆️",
        Direction::Down => "⬇️",
        Direction::Left => "⬅️",
        Direction::Right => "➡️"
    }
}

fn main() {
    let go_up = which_way(Direction::Up);
    let go_left = which_way(Direction::Left);
    let go_down = which_way(Direction::Down);
    let go_right = which_way(Direction::Right);

    println!("{go_up} {go_up} {go_down} {go_down} {go_left} {go_right} {go_left} {go_right} 🅱️ 🅰️");
}