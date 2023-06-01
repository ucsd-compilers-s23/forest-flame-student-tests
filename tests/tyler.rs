mod infra;

// Your tests go here!
success_tests! {
    {
        name: writeup_example_no_limit,
        file: "writeup_example.snek",
        expected: "[nil, [1, 2], nil]\nnil\n[1, 2, 3, 4]"
    },
    {
        name: writeup_example_15_heap_limit,
        file: "writeup_example.snek",
        input: "false",
        heap_size: 15,
        expected: "[nil, [1, 2], nil]\nnil\n[1, 2, 3, 4]"
    },
    {
        name: memory_realloc_9_limit,
        file: "memory_realloc.snek",
        input: "false",
        heap_size: 9,
        expected: "[0, 0]\n[2, 2, 2]\n[1, 1]\n[3, 3, 3]\n[2, 2]\n[4, 4, 4]\n[3, 3]\n[5, 5, 5]\n[4, 4]\n[6, 6, 6]\n[5, 5]\n[7, 7, 7]\nnil",
    },
    {
        name: bst_no_limit,
        file: "bst.snek",
        expected: "false\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\ntrue\ntrue\n0\n1\n2\n5\n10\n15\n16\n20\n25\n26\n27\ntrue",
    },
    {
        name: bst_70_limit,
        file: "bst.snek",
        input: "false",
        heap_size: 70,
        expected: "false\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\ntrue\ntrue\n0\n1\n2\n5\n10\n15\n16\n20\n25\n26\n27\ntrue",
    },
    {
        name: vec_bad_reverse,
        file: "vec_bad_reverse.snek",
        input: "50",
        expected: "[49, [48, [47, [46, [45, [44, [43, [42, [41, [40, [39, [38, [37, [36, [35, [34, [33, [32, [31, [30, [29, [28, [27, [26, [25, [24, [23, [22, [21, [20, [19, [18, [17, [16, [15, [14, [13, [12, [11, [10, [9, [8, [7, [6, [5, [4, [3, [2, [1, [0, nil]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]",
    },
    {
        name: vec_bad_reverse_limit,
        file: "vec_bad_reverse.snek",
        input: "50",
        heap_size: 2020,
        expected: "[49, [48, [47, [46, [45, [44, [43, [42, [41, [40, [39, [38, [37, [36, [35, [34, [33, [32, [31, [30, [29, [28, [27, [26, [25, [24, [23, [22, [21, [20, [19, [18, [17, [16, [15, [14, [13, [12, [11, [10, [9, [8, [7, [6, [5, [4, [3, [2, [1, [0, nil]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]",
    },
    {
        name: tomfoolery_no_limit,
        file: "tomfoolery.snek",
        input: "false",
        expected: "nil",
    },
    {
        name: tomfoolery_limit,
        file: "tomfoolery.snek",
        input: "false",
        heap_size: 12,
        expected: "nil",
    },
    {
        name: dynarr_no_limit,
        file: "dynarr.snek",
        expected: "[1, 10, nil]\n[2, 10, 20]\n[3, 10, 20, 30, nil]\n[4, 10, 20, 30, 40]\n[5, 10, 20, 30, 40, 50, nil, nil, nil]\n[6, 10, 20, 30, 40, 50, 60, nil, nil]\n[7, 10, 20, 30, 40, 50, 60, 70, nil]\n[8, 10, 20, 30, 40, 50, 60, 70, 80]\n8"
    },

    {
        name: dynarr_limit,
        file: "dynarr.snek",
        input: "false",
        heap_size: 18,
        expected: "[1, 10, nil]\n[2, 10, 20]\n[3, 10, 20, 30, nil]\n[4, 10, 20, 30, 40]\n[5, 10, 20, 30, 40, 50, nil, nil, nil]\n[6, 10, 20, 30, 40, 50, 60, nil, nil]\n[7, 10, 20, 30, 40, 50, 60, 70, nil]\n[8, 10, 20, 30, 40, 50, 60, 70, 80]\n8"
    },
    {
        name: gc_on_nothing,
        file: "gc_on_nothing.snek",
        expected: "nil",
    },
    {
        name: cycle_gc_no_limit,
        file: "cycle_gc.snek",
        expected: "[1, 2, 3, [4, 5, 6, [...]]]",
    },
    {
        name: nested_gc_no_limit,
        file: "nested_gc.snek",
        expected: "[0]\n[1]\n[2]\n[3]\n[4]\n[5]\n[6]\n[6]",
    },
    {
        name: nested_gc_limit,
        file: "nested_gc.snek",
        input: "false",
        heap_size: 26,
        expected: "[0]\n[1]\n[2]\n[3]\n[4]\n[5]\n[6]\n[6]",
    }
}

runtime_error_tests! {
    {
        name: writeup_example_oom,
        file: "writeup_example.snek",
        input: "false",
        heap_size: 14,
        expected: "out of memory",
    },
    {
        name: memory_realloc_oom,
        file: "memory_realloc.snek",
        input: "false",
        heap_size: 8,
        expected: "out of memory"
    },
    {
        name: bst_oom,
        file: "bst.snek",
        input: "false",
        heap_size: 69,
        expected: "out of memory"
    },
    {
        name: vec_bad_reverse_oom,
        file: "vec_bad_reverse.snek",
        input: "50",
        heap_size: 1000,
        expected: "out of memory",
    },
    {
        name: tomfoolery_oom,
        file: "tomfoolery.snek",
        input: "false",
        heap_size: 11,
        expected: "out of memory",
    },
    {
        name: cycle_gc_make_sure_no_clear,
        file: "cycle_gc_make_sure_no_clear.snek",
        input: "false",
        heap_size: 12,
        expected: "out of memory",
    },
    {
        name: nested_gc_oom,
        file: "nested_gc.snek",
        input: "false",
        heap_size: 25,
        expected: "out of memory"
    }
}

static_error_tests! {}
