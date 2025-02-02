/*

Parses and transform a text file.

Input format description markup:
<date=iso8601date> <time="M"|"K">: <systolic=u8>/<diastolic=u8> <pulse=u8>

*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
}
