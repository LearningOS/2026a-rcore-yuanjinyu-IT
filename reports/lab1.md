# 实现思路
1. 在 TaskManagerInner 中增加按任务划分的系统调用计数数组。
2. 在 syscall 分发函数中先记录调用次数，再执行对应系统调用。
3. 实现 sys_trace 的三种操作，并对非法请求返回 -1。
4. 使用原始指针完成单字节内存读写。
5. 保证不同任务的调用统计互不影响。

--- 

# 简答题

1. U 态特权操作测试

U 态不能直接执行 S 态特权指令或访问受限 CSR。执行不允许的操作时，处理器产生异常，进入内核 Trap 处理程序。

实际使用的 SBI 及版本：RustSBI v0.4.1

三个 bad 测例的执行结果：

- （1）ch2b_bad_address.rs

该程序尝试通过 write_volatile 向地址 0x0 写入一个字节。由于该地址不允许用户程序写入，预计触发 StoreFault 异常，对应异常码为 7。内核捕获异常后，应终止当前任务。

- （2）ch2b_bad_instruction.rs

该程序在 U 态执行 sret 指令。由于 sret 属于特权指令，U 态不允许执行，因此预计触发 IllegalInstruction 异常，对应异常码为 2。

- （3）ch2b_bad_register.rs

该程序尝试通过 csrr 指令读取 sstatus 寄存器。由于 sstatus 是 S 态控制与状态寄存器，U 态无法直接读取，因此预计触发 IllegalInstruction 异常，对应异常码为 2。


2. trap.S 分析

（1）刚进入 __restore 时 sp 的含义

sp 指向内核栈上的 TrapContext。该函数既用于首次启动用户任务，也用于 Trap 处理完成后恢复用户任务。

（2）特殊处理的寄存器

sstatus 用于恢复处理器状态及返回特权级相关信息；sepc 指定返回地址；sscratch 用于保存用户栈指针，供最后恢复使用。

（3）跳过 x2 和 x4 的原因

x2 是栈指针，需要在最后恢复。x4 是线程指针，当前实验的用户程序不使用它，因此跳过。

（4）__restore 中执行 csrrw 后

sp 保存用户栈指针，sscratch 保存内核栈指针。

（5）特权级切换指令

通过 sret 完成。由于 sstatus.SPP 被设置为 U 态，执行后返回 U 态。

（6）__alltraps 中执行 csrrw 后

sp 保存内核栈指针，sscratch 保存用户栈指针。

（7）从 U 态进入 S 态

由硬件 Trap 机制完成，例如执行 ecall 后，处理器根据异常委托配置进入 S 态，随后执行 stvec 指定的入口代码。

