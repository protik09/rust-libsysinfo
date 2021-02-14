#![crate_name = "doc"]
// Grab CPU info
// author: Protik Banerji <protik09@gmailcom>

struct cpu_info_t
{
    processor: i16;
    vendor_id: String;
    cpu_family: i16;
    model: i16;
    mode_name: String;
    stepping: i8;
    microcode: i16;
    cpu_mhz: f32;
    
}
// Create an empty and growable `String`
let mut string = String::new();

/// Read the CPU info and parse into human readable text
fn