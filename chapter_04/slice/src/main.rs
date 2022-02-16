fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    //第十三章详细讨论迭代器。
    //现在，只需知道 iter 方法返回集合中的每一个元素，
    //enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。
    for (i, &item) in bytes.iter().enumerate() {
        //相同的方式获取单词结尾的索引，通过寻找第一个出现的空格。
        //当找到一个空格，我们返回一个字符串 slice，
        //它使用字符串的开始和空格的索引作为开始和结束的索引。
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello_world _1");

    // first_word 中传入 `String` 的 slice
    //start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range
    //如果需要包含 end，可以使用 ..= 而不是 ..
    let word = first_word(&my_string[..]);
    println!("word_1 is {}", word);

    let my_string_literal = "hello world _2";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);
    println!("word_2 is {}", word);

    // 因为字符串字面值就是字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);

    println!("word_3 is {}", word);
}
