#!/bin/bash

# Part 1
mkdir -p part1_async_rust
cd part1_async_rust
crates1=(
  "01_why_async" "02_threads_vs_async" "03_future_trait" "04_async_await_syntax"
  "05_pin_unpin" "06_executors_runtimes" "07_async_error_handling" "08_spawning_tasks"
  "09_join_select" "10_async_streams" "11_async_channels" "12_sync_primitives"
  "13_timeouts_cancellation" "14_joinset" "15_custom_future"
)
for c in "${crates1[@]}"; do
  cargo new $c --name "p1_$c"
done
cd ..

# Part 2
mkdir -p part2_tokio
cd part2_tokio
crates2=(
  "01_tokio_setup" "02_tokio_runtime" "03_tokio_io_traits" "04_tcp_networking"
  "05_tokio_time" "06_task_management" "07_channels_deep_dive" "08_tokio_streams"
  "09_error_handling_tasks" "10_graceful_shutdown" "11_tracing_observability" "12_testing_async"
  "13_performance_tuning" "14_tokio_console"
)
for c in "${crates2[@]}"; do
  cargo new $c --name "p2_$c"
done
cd ..
