

pub fn convert_to_title(column_number: i32) -> String {
    let mut column_number = column_number;
    let shang = -1;
    let mut _yushu = 0;
    let mut yushu_arr: Vec<i32> = vec![];
    // 余数为零的情况代表进位
    while column_number!=0 {
        _yushu = column_number % 26;
        if _yushu==0 {
            _yushu=26;
            column_number-=1;
        }
        yushu_arr.push(_yushu);
        column_number /= 26
    }
    yushu_arr.reverse();
    let c:String=yushu_arr.iter().map(|&n|(n as u8 +b'A'-1) as char).collect();
    println!("{:?}",yushu_arr);
    println!("{:?}",c);
    c
}
