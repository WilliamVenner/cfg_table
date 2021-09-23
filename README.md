# ✨ `cfg_table`

A simple macro that expands to different values across compilation targets.

## Example

```rust
#[macro_use]
extern crate cfg_table;

let var = cfg_table! {
	_ => 123, // default value if nothing matches, this must be at the top

	[all(target_os = "freebsd", target_pointer_width = "64", feature = "my-feature")] => 1337, // custom

	// common platforms
	win32 => 32,
	win64 => 64,
	linux32 => 32,
	linux64 => 64,
	macos32 => 32,
	macos64 => 64,

	// pointer widths
	32 => 1985,
	"32" => 1985,
	64 => 2003,
	"64" => 2003
};

cfg_table! {
	_ => {
		panic!("What the heck is a \"Linux\"?");
	},

	win32 => {
		println!("You're on Windows 32-bit!");
	},

	win64 => {
		println!("You're on Windows 64-bit!");
	}
};
```