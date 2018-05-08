# csvgen

[![codecov](https://codecov.io/gh/joppich/csvgen/branch/master/graph/badge.svg)](https://codecov.io/gh/joppich/csvgen)
[![Build Status](https://travis-ci.com/joppich/csvgen.svg?branch=travis)](https://travis-ci.com/joppich/csvgen)

a small utility to create mock-csv files with various datatypes.

## commandline parameters

All commandline parameters are optional.

- __size__: required filesize

  default: _2 MB_
- __header__: create column headers

  default: _string, float, int_
- __column-types__: pass types for column values

  default: _string, float, int_
- __filepath__: write to provided path

  default: _stdout_

## examples

### display help

```lang=bash
$ csvgen --help
csvgen 0.1.0
jkm <jkm@bricknet.de>
Generate mock csv-files with various datatypes.

USAGE:
    csvgen [FLAGS] [OPTIONS]

FLAGS:
        --help       Prints help information
    -h, --header     Print column headers.
    -V, --version    Prints version information

OPTIONS:
    -c, --column-types <Column Types>    Specify types for columns.
    -f, --filepath <File path>           Write to file path instead of stdout.
    -s, --size <Size>                    Specify size of output file in MB.
```

### write 1 MB to stdout using default output-format

```lang=bash
$ csvgen -s 1 | tail -5
"pass cute",234.63565236879606,-1574574
"modern pleasure found various",-370.60698067726923,172070
"burn vegetable modern various",-257.9846409797992,-155605
"knowing",515.0382860189012,-1358241
"explain thank dress",500.2067882269995,1523743
```

### write default-sized (2 MB), custom-formatted file at specified path and print header-line

```lang=bash
$ csvgen -h -c int,string,int,string,int -f /tmp/foo.csv
$ head -5 /tmp/foo.csv
int,string,int,string,int
935304,"knowing explain burn sofa",-219593,"sofa sofa",1119124
-1141850,"dress modern vegetable knowing",-740429,"compare cute agree",861869
-1628699,"dress sofa grateful",-1109470,"agree explain",632448
-684272,"cooing tail",639600,"dress",1110599
```
