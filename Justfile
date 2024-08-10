run-book:
    mdbook serve ./book --open

run-lesson:
    cargo run

build-book:
    mdbook build ./book

fmt:
    cargo fmt

clean_book_cache:
    rm -rf ./book/book
    rm -rf ./docs

install:
    cargo install mdbook