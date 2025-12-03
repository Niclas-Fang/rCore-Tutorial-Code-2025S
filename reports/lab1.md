# lab1.md

## 实现功能

1. 为crate::task::TaskControlBlock增加了syscall_times字段，并实现了incr_syscall_times和get_syscall_times方法，
1. 为crate::task::TaskManager实现了incr_current_task_syscall_times和get_current_task_syscall_times方法，
1. 利用crate::task::TASK_MANAGER静态实例实现了increase_syscall_times和get_current_syscall_times方法，
1. 在crate::syscall::syscall里添加了increase_syscall_times方法以计数
1. 实现crate::syscall::process::sys_trace

## 简答作业

1. bad示例
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
sbi版本：RustSBI-QEMU Version 0.2.0-alpha.2
2. trap.S理解
    1. sp是stack pointer即为栈指针，指向栈顶，__restore主要用于从用户态trap进内核态后恢复用户态的上下文并返回用户态，比如在系统调用的时候和中断的时候
    2. 处理了t0,t1,t2和sstatus,sepc,sscratch，其中sscratch保存了x[2]即栈指针；sstatus保存了特权级，用于标记；spec保存了异常前最后一条指令的地址，用于恢复继续执行trap前的指令
    3. x2:它指向的是内核栈 x4:用不上
    4. sp->内核栈顶, sscratch->用户栈顶
    5. sret, 可以设置sstatus的值为U，并且跳转到spec指向的位置
    6. sp->内核栈顶, sscratch->用户栈顶
    7. ecall

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与以下各位就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无
2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
