pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut v = Vec::<Vec<i32>>::new();
    for _i in 0..num_rows {
        v.push(vec![1]);
    }
    if v.len()>=2 {
        v[1].push(1);
    }
    
    for x in 2..num_rows as usize {
        let (head,tail)=v.split_at_mut(x);
        let prev_row = &head[x - 1]; // 不可变借用前一行
        println!("head{:?}  tail{:?}",head,tail);
        let current_row = &mut tail[0]; // 可变借用当前行
        for y in 1..prev_row.len() {            
            current_row.push(prev_row[y-1]+prev_row[y]);
        }
        current_row.push(1);
    }
    v
}
