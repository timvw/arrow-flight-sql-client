# Arrow Flight SQL Client

flightsqlclient is a small command line app to query a [Flight SQL server](https://arrow.apache.org/blog/2022/02/16/introducing-arrow-flight-sql/).
You can find instructions to run such a server here: [notes on running java FlightSqlExample](http://timvw.be/2022/04/28/notes-on-running-java-flightsqlexample/).

## Installation

```bash
brew tap timvw/tap
brew install arrow-flight-sql-client
```

```bash
cargo install arrow-flight-sql-client
```

## Usage

When you run the app without arguments, you will be greeted with it's usage:

```bash
cargo run

USAGE:
    client <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    execute
    get-catalogs
    get-exported-keys
    get-imported-keys
    get-primary-keys
    get-schemas
    get-table-types
    get-tables
    help                 Print this message or the help of the given subcommand(s)
```

Listing the available tables can be done as following:

```
cargo run -- get-tables --hostname localhost --port 52358

+--------------+----------------+------------------+--------------+
| catalog_name | db_schema_name | table_name       | table_type   |
+--------------+----------------+------------------+--------------+
|              | SYS            | SYSALIASES       | SYSTEM TABLE |
... (removed some lines to reduce output)
|              | APP            | FOREIGNTABLE     | TABLE        |
|              | APP            | INTTABLE         | TABLE        |
+--------------+----------------+------------------+--------------+
```

A query can be executed as following:

```
cargo run -- execute --query "select * from app.inttable order by value desc"

+----+--------------+-------+-----------+
| ID | KEYNAME      | VALUE | FOREIGNID |
+----+--------------+-------+-----------+
| 1  | one          | 1     | 1         |
| 2  | zero         | 0     | 1         |
| 3  | negative one | -1    | 1         |
+----+--------------+-------+-----------+
```
