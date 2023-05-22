# jcd
```
A simple command line tool to detector the conflict classes in jars

Usage: jcd [OPTIONS] --jars <JAR_LIST>

Options:
  -j, --jars <JAR_LIST>        The jar list joined by semicolon
      --disable-crc            Disable the crc check
  -e, --exclude <EXCLUDE>      The exclude package prefix
  -h, --help                   Print help
  -V, --version                Print version

```

# Example

```shell
jcd --jars "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude 
```

```shell
jcd --jars "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude --disable-crc 
```

By default, the class only recognized as conflicted when it has the same class name and the crc number is not equal, 
You can disable it by `--disable-crc`

# Todo

- [ ] Multi thread processing
- [ ] Output to file
- [ ] Specify the output format, eg: json
- [ ] Maybe some benchmark
