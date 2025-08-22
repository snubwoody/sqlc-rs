# sqlc
A sql library.

## Not an ORM
This is **not** an ORM, it has no requirements on how you structure your code. Nor will it 
make migrations for you.

## Data types
| Sql                   | Rust                          |
|-----------------------|-------------------------------|
| `INT`                 | `i32`                         |
| `BIGINT`              | `i64`                         |
| `SMALLINT`            | `i16`                         |
| `SERIAL`              | `i32`                         |
| `CHAR`                | `String`                      |
| `TEXT`                | `String`                      |
| `VARCHAR`             | `String`                      |
| `BOOLEAN`             | `bool`                        |
| `TIME`                | --                            |
| `TIMESTAMP`           | --                            |
| `TIMESTAMPTZ`         | --                            |
| `INTERVAL`            | --                            |
| `UUID`                | --                            |
| `BYTEA`               | `Box<u8>` \ `&'static [u8]` ? |
| `JSON`                | --                            |
| `ENUM`                | --                            |
| `DECIMAL` \ `NUMERIC` | `f64`                         |
| `MONEY`               | `f32`                         |


--: Not yet supported

## TODO
- Remove plural of table names