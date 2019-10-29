#[macro_use] 
extern crate lazy_static;

extern crate regex;

mod static_kv {
    use std::collections::HashMap;
    use std::sync::RwLock;

    // 定义了公开的普通常量NF，它是一个脂肪层字面量类型
    // 如果需要使用，就必须带上命名空间， static_kv::NF
    pub const NF: &'static str = "not found";

    // 使用lazy_static!宏定义了两个全局静态变量MAP和MAP_MUT，分别代表只读的HashMap和可变的HashMap。
    /***
     * lazy_static!宏语法格式
     * 
     * lazy_static! {
     *  
     *      [pub static ref NAME_1 : TYPE_1 = EXPR_1;
     *      [pub] static ref NAME_1 : TYPE_1 = EXPR_1;
     * }
     * 
     * 必须严格按照此语法格式来书写，否则会引发线程恐慌
     */
    lazy_static! {
        pub static ref MAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m
        };

        pub static ref MAP_MUT : RwLock<HashMap<u32, &'static str>> = {
            let mut m = HashMap::new();
            m.insert(0, "bar");
            RwLock::new(m)
        };
    }
}

fn read_kv() {
    let ref m = static_kv::MAP;
    assert_eq!("foo", *m.get(&0).unwrap_or(&static_kv::NF));

    assert_eq!(static_kv::NF, *m.get(&1).unwrap_or(&static_kv::NF));
}

fn rw_mut_kv() -> Result<(),String> {
    {
        let m = static_kv::MAP_MUT.read().map_err(|e| e.to_string())?;

        assert_eq!("bar", *m.get(&0).unwrap_or(&static_kv::NF));
    }
    {
        let mut m = static_kv::MAP_MUT.write().map_err(|e| e.to_string())?;
        m.insert(1, "baz");
    }
    Ok(())
}

fn main() {
    println!("static hashmap");
    read_kv();

    match rw_mut_kv() {
        Ok(()) => {
            let m = static_kv::MAP_MUT.read().map_err(|e| e.to_string()).unwrap();
            assert_eq!("baz", *m.get(&1).unwrap_or(&static_kv::NF));
        },
        Err(e) => {println!("Error {}",e)},
    }
}