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

const PROGRAM_SOURCE: &str = include_str!("aoc.opencl");

const KERNEL_NAME: &str = "aoc_year2022_day4";

fn main() -> Result<()> {
    let context = create_opencl_context();
    let queue = create_opencl_queue(&context);
    let kernel = create_opencl_kernel(&context);

    let puzzle = include_str!("../../../../puzzles/year2022_day4.txt");
    let (range_1_lower_data,
        range_1_upper_data,
        range_2_lower_data,
        range_2_upper_data) = util::text_to_vecs(puzzle);
    let input_size = range_1_lower_data.len();

    let mut range_1_lower_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut range_1_upper_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut range_2_lower_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut range_2_upper_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let results_1_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, input_size, ptr::null_mut())? };
    let results_2_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, input_size, ptr::null_mut())? };

    let range_1_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut range_1_lower_buffer, CL_NON_BLOCKING, 0, &range_1_lower_data, &[])? };
    let range_1_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut range_1_upper_buffer, CL_NON_BLOCKING, 0, &range_1_upper_data, &[])? };
    let range_2_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut range_2_lower_buffer, CL_NON_BLOCKING, 0, &range_2_lower_data, &[])? };
    let range_2_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut range_2_upper_buffer, CL_NON_BLOCKING, 0, &range_2_upper_data, &[])? };

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&range_1_lower_buffer)
            .set_arg(&range_1_upper_buffer)
            .set_arg(&range_2_lower_buffer)
            .set_arg(&range_2_upper_buffer)
            .set_arg(&results_1_buffer)
            .set_arg(&results_2_buffer)
            .set_global_work_size(input_size)
            .set_wait_event(&range_1_lower_write_event)
            .set_wait_event(&range_1_upper_write_event)
            .set_wait_event(&range_2_lower_write_event)
            .set_wait_event(&range_2_upper_write_event)
            .enqueue_nd_range(&queue)?
    };

    let events: Vec<cl_event> = vec![kernel_event.get()];
    let mut results_1: Vec<cl_int> = vec![0; input_size];
    let mut results_2: Vec<cl_int> = vec![0; input_size];
    let read_event_1 = unsafe { queue.enqueue_read_buffer(&results_1_buffer, CL_NON_BLOCKING, 0, &mut results_1, &events)? };
    let read_event_2 = unsafe { queue.enqueue_read_buffer(&results_2_buffer, CL_NON_BLOCKING, 0, &mut results_2, &events)? };
    read_event_1.wait()?;
    read_event_2.wait()?;

    let r1: i32 = results_1.into_iter().sum();
    let r2: i32 = results_2.into_iter().sum();
    println!("results part 1: {}, part 2: {}", r1, r2);
    // part1 444
    // part2 801

    println!("kernel execution duration (ns): {}", kernel_duration(kernel_event));

    Ok(())
}

fn create_opencl_queue(context: &Context) -> CommandQueue {
    CommandQueue::create_default_with_properties(context, CL_QUEUE_PROFILING_ENABLE, 0)
        .expect("CommandQueue::create_default_with_properties failed")
}

fn create_opencl_kernel(context: &Context) -> Kernel {
    let program = Program::create_and_build_from_source(context, PROGRAM_SOURCE, "")
        .expect("Program::create_and_build_from_source failed");
    Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed")
}

fn create_opencl_context() -> Context {
    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)
        .expect("get_all_devices failed")
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    Context::from_device(&device).expect("Context::from_device failed")
}

fn kernel_duration(kernel_event: Event) -> cl_ulong {
    let start_time = kernel_event.profiling_command_start().unwrap();
    let end_time = kernel_event.profiling_command_end().unwrap();
    
    end_time - start_time
}
