fn main() {
    //所有权：
    //steak（LIFO） heap()
    //steak上存数据必须拥有已知且固定的大小，其余的必须放heap
    //heap内存组织性差一些
    //值压到stack上不叫分配，由于指针大小固定，可以存于stack
    //数据压到stack上快于heap上分配：因操作系统不需要寻找用于存储新数据的空间，其位置永远处于顶端
    //heap反之
    //访问heap数据要用指针，所以慢（对现代处理器，若指令在内存中跳转次数越少，速度越快）
    //代码调用函数时，值传入（包括指向heap的指针）。函数本地变量被压到stack上。函数结束，值弹出
    //所有权解决的问题：跟踪代码哪部分在用heap的那些数据，最小化heap上重复数据量。清理heap上未使用数据

    //每个值都有一个变量，变量即该值所有者
    //每个值同时只能有一个所有者
    //所有者超出作用域（scope）时该值被删除

    //string类型分配于heap，可以存储在编译时位置数量的文本
    //用from从字符串字面值创建string

    //rust有特性为“引用（reference）”
    //引用与常量一样默认不可变
    //但是&mut可变
    
    let my_s = String::from("hello world");
    let word_index = first_word(&my_s[..]);








}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}