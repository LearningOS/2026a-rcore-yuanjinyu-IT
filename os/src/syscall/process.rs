//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}


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