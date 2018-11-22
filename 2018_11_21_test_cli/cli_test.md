instead of clirunner, ideas for user-run-like test cases:
- use <root>/tests/
- create functions for
    - test exit code == 0
    - test image size == X before and Y after
    - test based on header, it should be image format X
    - test based on

-  commands take
    - input file
    - output file name
    - output file extension (list?)
    - list of test functions

    for example, command `cargo run -- input.png <output>_<r>.<ext> --pnm-encoding-ascii` could have
        input file <- resources/palette_4x4.png
        output file name <- pnm_encoding_ascii_true
        output file extension <- [__all__]
        list of test functions <-
            [
                is_ascii_encoded, expected true
            ]

        let mut r = lazy iterator u32
            .next
            .next
            .next

- to think about:
    - parallel vs sequential (i.e. don't save images with the same name)

- each test:
    - setup()
        - load image from resources/
        - generate image
        - set required var/vals
        -
    - test()
        - run Command with `cargo run -- <options>`
        - run test cases with pre defined functions

    - clean_up()
        - remove the created image file