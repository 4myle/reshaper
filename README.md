# Reshaper

Simple template-based parsing and transforming of value-based text files. 
User interface made with [egui](https://github.com/emilk/egui).

> **DISCLAIMER**: this application is a hobby project and should not be used for production use cases. It is provided as-is and is not likely to be maintained regurlarly.

## Use cases

The application can be used to reshape text files, like CSV-files or files delimited in other ways, to other formats. 
Example of source template to read a file:
```
<date> <time>: <systolic>/<diastolic> <pulse>
```
Blank here means "one or more white space characters". Input file can be like:
```
2024-10-25 M: 131/79 63
2024-10-25 K: 133/81 82
2024-10-26 M: 116/72 81
```

With a target template (to display and export) a file like:
```
<date>,<pulse>,<systolic>,<diastolic>
```
The output would be:
```
2024-10-25,63,131,79
2024-10-25,82,133,81
2024-10-26,81,116,72
```
As for now, only simple use cases like this are supported.

## Future

Possible extension to markup could be support for types and conversions. For example:
```
 <date="%Y-%m-%d"> <time="M"|"K">: <systolic=u8>/<diastolic=u8> <pulse=u8>
```
If so, expressions could be supported in the output format like,

```
 <date as "%m/%d/%Y">,<time as 1|2>,<systolic * 100 + distolic>,<pulse / 60>
```
