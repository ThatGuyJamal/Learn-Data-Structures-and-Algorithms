run-book:
    mdbook serve ./book --open

run-lesson:
    cargo run

build-book:
    mdbook build ./book --dest-dir ../docs # I named this docs so i can use github pages as the host...

fmt:
    cargo fmt

clean_book_cache:
    rm -rf ./book/book
    rm -rf ./docs

install:
    cargo install mdbook