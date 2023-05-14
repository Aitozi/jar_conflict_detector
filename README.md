# jcd
`jcd` is a simple command line tool to perform jar conflicts detector.

# Example

```shell
jcd --jar-list "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude 
```

```shell
jcd --jar-list "a.jar;b.jar" --exclude package1/to/exclude --exclude package2/to/exclude --disable-crc 
```

By default, the class only recognized as conflicted when it has the same class name and the crc number is not equal, 
You can disable it by `--disable-crc`

# Todo

- [ ] Multi thread processing
- [ ] Output to file
- [ ] Specify the output format, eg: json
- [ ] Maybe some benchmark
