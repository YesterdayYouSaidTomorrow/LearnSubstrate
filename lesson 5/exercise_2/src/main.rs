fn sum_integer_sets(t: &[u32]) -> Option<u32> {
    if t.len() > 0 {                         //判断数组是否为空
        let mut total:u32 = 0;               //定义一个可变的无符号32位整数类型变量 total
        for item in t.iter() {               //使用iter()迭代数组
            match total.checked_add(*item) {    //Checked integer addition. , 如果发生溢出则返回 None .
                Some(s) => total = s,           //匹配返回的值，如果存在，则取出数值赋予total
                None => return None,            //返回值为none,返回none
            } 
        }
        Some(total) //因为函数的返回值是option<u32>，所以用some封装total
    }
    else {
        None            //如果数组为空，直接返回none
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let arr2 = vec![];

    println!("array sum is {:?}",sum_integer_sets(&arr));
    println!("array sum is {:?}",sum_integer_sets(&arr2));

}