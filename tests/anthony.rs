mod infra;

success_tests! {
    subdir: "anthony",
    {
        name: cleanup_nested,
        file: "cleanup_nested.snek",
        input: "1000",
        heap_size: 4008,
        expected: "1000",
    },
    {
        name: gc_odd,
        file: "gc_odd.snek",
        input: "10000",
        heap_size: 40002,
        expected: "10001"
    },
    {
        name: gc_odd2,
        file: "gc_odd2.snek",
        input: "128",
        expected: "89"
    },
    {
        name: merge_sort,
        file: "merge_sort.snek",
        input: "1000",
        expected: "89"
    },
    {
        name: insertion_sort,
        file: "insertion_sort.snek",
        input: "1000",
        heap_size: 1002,
        expected: "true",
    },
}

runtime_error_tests! {
    subdir: "anthony",
    {
        name: insertion_sort_oom,
        file: "insertion_sort.snek",
        input: "1000",
        heap_size: 1001,
        expected: "out of memory",
    }
}

static_error_tests! {}
