#![crate_name = "doc"]
// Grab CPU info
// author: Protik Banerji <protik09@gmailcom>

// Structure contataining the CPU info stuff
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
    cache_size: i32;
    physical_id: i32;
    siblings: i32;
    core_id: i32;
    cpu_cores: i32;
    apicid: i32;
    initial_apicid: i32;
    fpu: bool;
    fpu_exception: bool;
    cpuid_level: i32;
    wp: bool;
    flags: Vec; // Vector of strings
    bugs: String;
    bogomips: f32;
    clflush_size: i32;
    cache_alignement: i32;
    address_sizes: String;
    power_management: String;
    
}
// Create an empty and growable `String`
let mut string = String::new();

/// Read the CPU info and parse into human readable text
fn