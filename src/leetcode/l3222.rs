pub fn losing_player(x: i32, y: i32) -> String {
    // 没有赢的玩家
    let mut single = 0;
    let mut x = x;
    let mut y = y;
    while x - 1 >=0 && y - 4 >=0 {
        x -= 1;
        y -= 4;
        single += 1;
    }
    if single % 2 == 0 {
        String::from("Bob")
    } else {
        String::from("Alice")
    }
}
