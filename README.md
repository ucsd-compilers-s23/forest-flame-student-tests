# forest-flame-student-tests

Congrats on making it so far! After forest-flame, we should now have a fully specced snek language! You should now be able to write a lot of awesome snek programs. If you would like to have access to extra test cases, this is the right place to look. If you have a test case you think can break a lot of your classmates compilers, this is the right place to put it. If you have a cool program or library that you want to share, we would love to see it here!

Some of you may not have buggy implementations of compilers and would like some more test cases. Before you use the test cases here, we highly recommend that you write your own test suite for for it. If you have a test case that really helped fix a bug for you, please add it to help your fellow classmates. If you think your test case would break a lot of your classmates compilers, share it with us through this repo as we would love to use it.

Currently, our language is missing basic features like a pseudorandom number generator and integer division. We can add these as functions in our language! If you think there are other features we are missing, feel free to write function implementations of them! You might also have some really cool programs such as a minigame or a fancy data structure that you want to share. Follow the instructions below for adding tests through pull requests.

## Adding your own tests

 1. Create a subdirectory with your name/username in the [tests](./tests) directory.
 1. Add your `.snek` test files to this directory.
 1. Add a file `<username>.rs` file (same format as forest-flame `tests/all_tests.rs`) inside `[tests/]` with success tests, static error tests, and runtime error tests. Make sure to specify the `subdir` attribute in each macro invocation pointing to the subdirectory you created in step 1.
 1. Make a pull request to this repo so that we can check and add your tests to this repo.

See [`tests/shubham.rs`](./tests/shubham.rs) and [`tests/shubham`](./tests/shubham) for an example of a test file and subdirectory.

Note: Please don't add the starter tests to your test folder when you make a pull request.
