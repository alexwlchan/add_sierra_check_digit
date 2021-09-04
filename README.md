# add-sierra-check-digit

This is a command-line tool for getting the check digit of a Sierra record number.

At [Wellcome], we use a library management system called [Sierra].
Within Sierra, records are identified by a [record number][record number], which is a seven-digit number with an optional record type prefix and modulo 11 check digit.

For example:

*   `1234567` is a record number
*   `12345672` is a record with a check digit
*   `b12345672` is a record with record type prefix and a check digit (here the "b" prefix means this is a bibliographic record)

I prefer to use the long form, with the record type prefix and check digit.
If you have the long form, it's easy to get to the short form.
If you have the short form, getting to the longer form is harder – and some contexts require the long form.

The Sierra documentation explains how to [work out the check digit][check digit]; this is a command-line tool works it out for me.
For example:

```console
$ add_sierra_check_digit 1234567
12345672
```

Here the check digit is "2".

[Wellcome]: https://github.com/wellcomecollection
[Sierra]: https://www.iii.com/products/sierra-ils/
[record number]: https://documentation.iii.com/sierrahelp/Default.htm#sril/sril_records_numbers.html
[check digit]: https://documentation.iii.com/sierrahelp/Default.htm#sril/sril_records_numbers.html#check_digit

## Usage

You can add a check digit to a seven-digit record number:

```console
$ add_sierra_check_digit 1962791
19627919
```

You can also add a check digit to a record number with a record type prefix, which will be preserved:

```console
$ add_sierra_check_digit i1962791
i19627919
```

Note that the check digit may not always be numeric: sometimes it's an "x".

```console
$ add_sierra_check_digit 1026579
1026579x
```

## Installation

You'll need Rust installed, then clone the repo and install it as a standard Rust binary:

```console
$ git clone git@github.com:alexwlchan/add_sierra_check_digit.git
$ cd add_sierra_check_digit
$ cargo install --force --path .
```

## Other implementations

I wrote this in Rust because this was a small, self-contained project where I could try Rust and get something super whizzy fast.

I've implemented the check digit algorithm in two other languages: [Python](add_check_digit.py) and [Scala][scala].

[scala]: https://github.com/wellcomecollection/scala-libs/blob/86d25fff221e9f918c819a0db5ff4673da174101/sierra/src/main/scala/weco/sierra/models/identifiers/TypedSierraRecordNumber.scala#L27-L52

## License

MIT.
