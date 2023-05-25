# jcd

A simple command line tool to detector the potential conflict classes in jars. 
The similar tool called [jarfish](https://code.google.com/archive/p/jarfish/wikis/Intro.wiki) in java.

```

Usage: jcd [OPTIONS] --jars <JAR_LIST>

Options:
  -j, --jars <JAR_LIST>    The jar list joined by semicolon
  -c, --check <CHECK>      [default: size] [possible values: crc, size, none]
  -e, --exclude <EXCLUDE>  The exclude package prefix
  -h, --help               Print help
  -V, --version            Print version

```

# Example

```shell
jcd --jars "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude 
```

```shell
jcd --jars "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude --disable-crc 
```

By default, the class only recognized as conflicted when it has the same class name but different size.
It can be tuned to check crc number `-c crc` or disable check with `-c none`. 

# Todo

- [ ] Multi thread processing
- [ ] Output to file
- [ ] Specify the output format, eg: json
- [ ] Maybe some benchmark
