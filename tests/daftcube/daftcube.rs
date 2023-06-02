mod infra;

// Your tests go here!
success_tests! {

    // My Tests
    {
        name: obar_lots_of_small_garbage,
        file: "owen_bartolf/lots_of_small_garbage.snek",
        input: "1000",
        expected: "0"
    },
    {
        name: obar_cyclic_vec,
        file: "owen_bartolf/cyclic_vec.snek",
        input: "1000",
        expected: "0"
    },
    {
        name: obar_cyclic_vec_chained,
        file: "owen_bartolf/cyclic_vec_chained.snek",
        input: "1000",
        expected: "0"
    },
    {
        name: obar_dag,
        file: "owen_bartolf/dag.snek",
        input: "1000",
        expected: " [[0, [1, [4, [5]], [2, [5]]]], [1, [4, [5]], [2, [5]]], [2, [5]], [3, [1, [4, [5]], [2, [5]]], [5]], [4, [5]], [5]]"
    },
    {
        name: obar_complex,
        file: "owen_bartolf/complex_graph.snek",
        input: "100",
        expected: "[[0, [1, [4, [5, [...]]], [2, [5, [...]]], [...]]], [1, [4, [5, [0, [...]]]], [2, [5, [0, [...]]]], [0, [...]]], [2, [5, [0, [1, [4, [...]], [...], [...]]]]], [3, [1, [4, [5, [0, [...]]]], [2, [5, [0, [...]]]], [0, [...]]], [...], [...], 0], [4, [5, [0, [1, [...], [2, [...]], [...]]]]], [5, [0, [1, [4, [...]], [2, [...]], [...]]]]]"
    },
    {
        name: obar_lots_of_functions,
        file: "owen_bartolf/lots_of_functions.snek",
        input: "1000",
        expected: "true"
    },
    {
        name: ref_multiple_times_stack,
        file: "owen_bartolf/ref_multiple_times_stack.snek",
        input: "1000",
        expected: "true"
    }
}

runtime_error_tests! {
}

static_error_tests! {}
