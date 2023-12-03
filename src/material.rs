use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Material {
    /// Cファイルの名前(拡張子なし)
    name: String,
    /// 依存しているヘッダファイルの名前(拡張子あり)
    dependencies: HashSet<String>,
}

impl Material {
    pub fn new(main_name: String, dependencies: HashSet<String>) -> Self {
        if main_name.ends_with(".c") {
            panic!("main_name must not end with .c");
        }

        Self {
            name: main_name,
            dependencies,
        }
    }

    pub fn object_name(&self) -> String {
        format!("{}.o", self.name)
    }

    pub fn c_name(&self) -> String {
        format!("{}.c", self.name)
    }

    pub fn dependencies_name(&self) -> String {
        let mut dependencies = String::new();
        for dependency in &self.dependencies {
            dependencies.push_str(&format!("{} ", dependency));
        }
        dependencies
    }

    pub fn gcc_command(&self) -> String {
        format!("gcc -c {} -o {}", self.c_name(), self.object_name())
    }
}
