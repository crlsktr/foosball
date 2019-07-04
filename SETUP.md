# Setup

## Diesel

install diesel_cli

```
cargo install diesel_cli --no-default-feature --features postgres
```

go into the foos directory

```
cd foos
```
then

```
diesel migration run --database-url=postgres://{username}:{password}@localhost/foosball
```

## Build

```
cargo build
```

## Run

```
cargo run -p foos_cli
```

or

```
cargo run -p foos_web
```