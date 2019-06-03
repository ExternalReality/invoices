#### Invoice

A simple in-memory invoice storage system

### Setup

0. Please install the Rust programming language: `https://www.rust-lang.org/tools/install` 

1. Clone the repository

```
git clone https://gitlab.com/ExternalReality1/invoices.git
```

2. CD into the repository

```
cd invoices
```

3. Get the dependencies

```
cargo fetch
```

### About the program

This program consists of three services:

* The invoice server
* The invoice analysis server
* the rating server

The invoice server stores the invoices
The invoice analysis server validates the invoices

currently the invoice analysis server only checks to see if a submitted invoice is a potential duplicate. That is, the invoice server will not allow "similar" invoices for a period of 15 minutes. After the 15 minutes have expired the invoice analysis server assumes that the invoice is legitimate (e.g. a repeate transaction of some sort).

### Try the program

To try the program you must first start both services. An easy way to do this is, from the project directory, run the following commands:

```
cargo run -p analysis_server
```

from a different terminal emulator run:

```
cargo run -p invoice_server
```


from yet a different terminal emulator run (you could always run processes in the background if you'd like):

```
cargo run -p rating_server
```


now you can run the command line tool to interact with the services. Again from the project directory (in another terminal) run:

```
cargo run --bin invoice -- list
```

You should see there are no invoices available (and empty list of invoice numbers).

```
cargo run --bin invoice -- submit -i examples/invoice.toml
```

Currently the system accepts toml encoded invoice files. You can look at a valid invoice in the `examples` directory. Try to submit the same invoice again.

```
cargo run --bin invoice -- submit -i examples/invoice.toml
```

Since this new submission is similar to the last, you should see a warning message appear indicating that the system thinks the submission is a possible mistake. Don't worry you can try again after 15 minutes and the system will accept your submission.

Sometimes you want to submit the invoice anyway because it is really just a duplicate order. You can do this with the `--force` option:

```
cargo run --bin invoice -- submit -i examples/invoice.toml --force
```

You may want to get the total amount of all submitted invoices:

```
cargo run --bin invoice -- bill <COMPANY NAME>
```


```
cargo run --bin invoice -- list
```

You should see invoice numbers for all your invoices.

Remove an invoice now:

```
cargo run --bin invoice -- remove <INVOICE NUMBER>
```


Listing the invoices again should reveal that the invoice has been removed.

### Error Handling

This is a learning project so no proper error handling on logging is done.


### Tests

You can run the tests with `cargo test --all`
