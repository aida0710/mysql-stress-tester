pub fn input_console(input: &mut String) {
    std::io::stdin()
        .read_line(input)
        .expect("lineの読み取りに失敗しました。");

    *input = input.trim_end().to_owned();
}
