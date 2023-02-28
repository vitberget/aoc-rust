use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{cl_event, cl_int, CL_NON_BLOCKING, cl_ulong};
use opencl3::Result;
use std::ptr;
use opencl3::event::Event;

mod util;

const PROGRAM_SOURCE: &str = include_str!("aoc.cl");
const KERNEL_NAME: &str = "aoc_year2022_day4";

fn main() -> Result<()> {
    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    let context = Context::from_device(&device).expect("Context::from_device failed");
    let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)
        .expect("CommandQueue::create_default failed");

    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "").expect("Program::create_and_build_from_source failed");
    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");

    let puzzle = include_str!("../../../puzzles/year2022_day4.txt");
    let (r1_lower_data, r1_upper_data, r2_lower_data, r2_upper_data) = util::text_to_vecs(puzzle);
    let input_size = r1_lower_data.len();

    let mut r1_lower_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r1_upper_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r2_lower_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r2_upper_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let results_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, input_size, ptr::null_mut())? };

    let r1_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut r1_lower_buffer, CL_NON_BLOCKING, 0, &r1_lower_data, &[])? };
    let r1_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut r1_upper_buffer, CL_NON_BLOCKING, 0, &r1_upper_data, &[])? };
    let r2_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut r2_lower_buffer, CL_NON_BLOCKING, 0, &r2_lower_data, &[])? };
    let r2_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut r2_upper_buffer, CL_NON_BLOCKING, 0, &r2_upper_data, &[])? };

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&r1_lower_buffer)
            .set_arg(&r1_upper_buffer)
            .set_arg(&r2_lower_buffer)
            .set_arg(&r2_upper_buffer)
            .set_arg(&results_buffer)
            .set_global_work_size(input_size)
            .set_wait_event(&r1_lower_write_event)
            .set_wait_event(&r1_upper_write_event)
            .set_wait_event(&r2_lower_write_event)
            .set_wait_event(&r2_upper_write_event)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    let mut results: Vec<cl_int> = vec![0; input_size];
    let read_event = unsafe { queue.enqueue_read_buffer(&results_buffer, CL_NON_BLOCKING, 0, &mut results, &events)? };
    read_event.wait()?;

    let r: i32 = results.into_iter().sum();
    println!("results front: {}", r);
    // part1 444
    // part2 801

    println!("kernel execution duration (ns): {}", kernel_duration(kernel_event));

    Ok(())
}

fn kernel_duration(kernel_event: Event) -> cl_ulong {
    let start_time = kernel_event.profiling_command_start().unwrap();
    let end_time = kernel_event.profiling_command_end().unwrap();
    let duration = end_time - start_time;
    return duration;
}
