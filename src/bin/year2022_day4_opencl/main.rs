use std::ops::RangeInclusive;
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{cl_event, cl_int, CL_BLOCKING, CL_NON_BLOCKING};
use opencl3::Result;
use std::ptr;

mod util;

const PROGRAM_SOURCE: &str = include_str!("aoc.cl");
const KERNEL_NAME: &str = "aoc_year2022_day4";

fn main() -> Result<()> {
    // Find a usable device for this application
    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    // Create a Context on an OpenCL device
    let context = Context::from_device(&device).expect("Context::from_device failed");

    // Create a command_queue on the Context's device
    let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)
        .expect("CommandQueue::create_default failed");

    // Build the OpenCL program source and create the kernel.
    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "")
        .expect("Program::create_and_build_from_source failed");
    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");

    /////////////////////////////////////////////////////////////////////
    // Compute data

    let puzzle = include_str!("../../../puzzles/year2022_day4.txt");
    let (r1_l, r1_h, r2_l, r2_h) = util::text_to_vecs(puzzle);
    let input_size = r1_l.len();

    // Create OpenCL device buffers
    let mut r1_lower = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r1_upper = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r2_lower = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let mut r2_upper = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, input_size, ptr::null_mut())? };
    let results_buffer = unsafe { Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, input_size, ptr::null_mut())? };


    // Non-blocking write, wait for y_write_event
    let r1_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut r1_lower, CL_NON_BLOCKING, 0, &r1_l, &[])? };
    let r1_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut r1_upper, CL_NON_BLOCKING, 0, &r1_h, &[])? };
    let r2_lower_write_event = unsafe { queue.enqueue_write_buffer(&mut r2_lower, CL_NON_BLOCKING, 0, &r2_l, &[])? };
    let r2_upper_write_event = unsafe { queue.enqueue_write_buffer(&mut r2_upper, CL_NON_BLOCKING, 0, &r2_h, &[])? };

    // Use the ExecuteKernel builder to set the kernel buffer and
    // cl_float value arguments, before setting the one dimensional
    // global_work_size for the call to enqueue_nd_range.
    // Unwraps the Result to get the kernel execution event.
    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&r1_lower)
            .set_arg(&r1_upper)
            .set_arg(&r2_lower)
            .set_arg(&r2_upper)
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

    // Create a results array to hold the results from the OpenCL device
    // and enqueue a read command to read the device buffer into the array
    // after the kernel event completes.
    let mut results: Vec<cl_int> = vec![0; input_size];
    let read_event = unsafe { queue.enqueue_read_buffer(&results_buffer, CL_NON_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    let r: i32 = results.into_iter().sum();

    // Output the first and last results
    println!("results front: {}", r);
    // part1 444
    // part2 801

    // Calculate the kernel duration, from the kernel_event
    let start_time = kernel_event.profiling_command_start()?;
    let end_time = kernel_event.profiling_command_end()?;
    let duration = end_time - start_time;
    println!("kernel execution duration (ns): {}", duration);

    Ok(())
}
