# Reshaper

Simple template-based parsing and transforming of a text file. 
User interface made with [egui](https://github.com/emilk/egui).

Example of input markup:
```
<date> <time>: <systolic>/<diastolic> <pulse>
```
Blank here means "one or more white space characters".

Output markup example:
```
<date>,<pulse>,<systolic>,<diastolic>
```
As for now, simple use cases like this are the only ones supported.

## Some ideas

Possible extension to markup could be support for type and conversions. For example:
```
 <date="%Y-%m-%d"> <time="M"|"K">: <systolic=u8>/<diastolic=u8> <pulse=u8>
```
If so, expression could be supported in the output format, like.

```
 <date as "%m/%d/%Y">,<time as 1|2>,<systolic*100 + distolic>,<pulse/60>
```
Date parsing strings here are the ones support by [chrono](https://docs.rs/chrono/latest/chrono/). 

Support for multi-line input by aggregating on a choosen variable would also be useful.
