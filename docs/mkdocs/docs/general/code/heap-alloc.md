# Heap Fragmentation

The heap is an area of memory normally used for dynamic memory access.
This includes types such as the String or Vec which can dynamically allocate space at runtime.

One of the issues with dynamic memory allocation can be memory fragmentation

  * <https://www.reddit.com/r/rust/comments/12bmah6/spotting_and_avoiding_heap_fragmentation_in_rust/>
  * <https://www.svix.com/blog/heap-fragmentation-in-rust-applications/>

Historically rust used jemalloc
But this was removed later on in favour of a rust based system allocator

  * <https://www.reddit.com/r/rust/comments/9twam5/jemalloc_was_just_removed_from_the_standard/>

## Embedded Platforms

With embedded platforms that are `nostd` by default there's no allocator for the heap or dynamic memory allocation
So we have to provide our own.

The allocator can be specified via `#[global_allocator]`

  * <https://internals.rust-lang.org/t/option-to-add-replacement-std-dependency-to-subdependencys-for-no-std-platforms/22661/12>
  * <https://github.com/nostd-rs/nostd>
  * <https://github.com/rust-embedded/embedded-alloc>

One comment
```
And about alloc, there is a good reason to avoid it - on devices without MMU, your heap will soon become fragmented and your program will crash (OOM).
This is not an issue for non-realtime 64 bit systems with MMU because fragmentation is limited to a single page, anything higher can be remapped freely.
```

  * <https://www.reddit.com/r/embedded/comments/16vxxfm/fragmentation_in_baremetal_vs_os/>
  * <https://stackoverflow.com/questions/40658045/does-rusts-memory-management-result-in-fragmented-memory>

The ESP32 does have an MMU but the RP2040 / RP2350 does not have an MMU
with STM32 devices those that have an MMU are typically Application level processors not the M type
This probably explains why the ESP32 has access to the std library
