//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Syscall counter
    pub syscall_times: [usize; 500],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// Increase the syscall counter
    pub fn incr_syscall_times(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
    /// Get the syscall time of a syscall
    pub fn get_syscall_times(&self, syscall_id: usize) -> usize {
        self.syscall_times[syscall_id]
    }
}
