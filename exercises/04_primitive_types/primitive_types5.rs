fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: 在一条语句中解构 `cat` 元组，使得 `println` 语句能够正常工作。
    let name = cat.0;
    let age  = cat.1;
    println!("{name} 今年 {age} 岁了");
}
