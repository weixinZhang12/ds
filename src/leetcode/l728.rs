pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut num_split_array:Vec<i32>=vec![];
    let mut _temp = 0;
    let mut _flag: bool=false;
    for i in left..=right {
        let mut num = i;
        while num != 0 {
            _temp=num%10;
            num/=10;
            num_split_array.push(_temp);
            // temp=0;
        }
        _flag=true;
        for j in num_split_array {
           
            if j==0||i%j!=0 {
                _flag=false;
                break;
            }
        }
        if _flag {
            v.push(i);
        }
        num_split_array=vec![];
    }
    v
}
