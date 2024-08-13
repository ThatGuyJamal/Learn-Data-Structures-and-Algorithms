run-book:
    mdbook serve ./book --open

run-lesson:
    cargo run

build-book:
    mdbook build ./book
    npm run inject-analytics
    @echo "Book ready to deploy!"

fmt:
    cargo fmt

clean_book_cache:
    rm -rf ./book/book
    rm -rf ./docs

install:
    cargo install mdbook