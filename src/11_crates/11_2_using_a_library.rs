/*
 * Using a library
 *
 * To link a crate to this new library you may use `rustc`'s `--extern` flag.
 * All of its items will then be imported under a module named the same as the
 * library. This module generally behaves the same way as any other module.
 */

// extern crate rary; // May be required for Rust 2015 edition or earlier
// extern crate 11_1_creating_a_library;

fn main() {
  rary::public_function();
  // 11_1_creating_a_library::public_function();

  // Error! `private_function` is private
  //rary::private_function();
  //11_1_creating_a_library::private_function();

  rary::indirect_access();
  // 11_1_creating_a_library::indirect_access();
}

// ```
// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`
// ```
