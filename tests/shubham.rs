mod infra;

success_tests! {
    subdir: "shubham",
    {
        name: twenty_one,
        file: "twenty_one.snek",
        expected: "19",
    },
}

runtime_error_tests! {}

static_error_tests! {}
