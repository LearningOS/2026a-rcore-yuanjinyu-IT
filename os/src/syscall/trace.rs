use crate::task::get_syscall_count;

pub fn sys_trace(
    trace_request: usize,
    id: usize,
    data: usize,
) -> isize {
    match trace_request {
        // 读取一个字节
        0 => unsafe {
            (id as *const u8).read_volatile() as isize
        },

        // 写入一个字节
        1 => {
            unsafe {
                (id as *mut u8).write_volatile(data as u8);
            }
            0
        },

        // 查询系统调用次数
        2 => get_syscall_count(id) as isize,

        // 不支持的操作
        _ => -1,
    }
}