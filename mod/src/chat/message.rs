pub struct Message {
    id: u32,
    content: String,
}

impl Message {
    // 关联函数。可以理解为 Swift 中的 “类方法、静态方法”
    pub fn new() -> Self {
        Self {
            id: 1,
            content: "hello world".to_string(),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.id += self.id;
        self.content = content;
    }

    // 方法. 可以理解为 Swift 中的 “实例方法、对象方法”
    pub fn print_message(&self) {
        // Rust 宏
        println!("id: {}, content: {}", self.id, self.content);
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}
