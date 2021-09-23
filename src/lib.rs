#![doc = include_str!("../README.md")]

#[macro_export]
#[doc = include_str!("../README.md")]
macro_rules! cfg_table {
	(@expand $cfg_table:tt 64 => $expr:expr) => {
		#[cfg(target_pointer_width = "64")] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt 32 => $expr:expr) => {
		#[cfg(target_pointer_width = "32")] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt "64" => $expr:expr) => {
		#[cfg(target_pointer_width = "64")] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt "32" => $expr:expr) => {
		#[cfg(target_pointer_width = "32")] {
			break $cfg_table $expr;
		}
	};

	(@expand $cfg_table:tt linux => $expr:expr) => {
		#[cfg(target_os = "linux")] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt win => $expr:expr) => {
		#[cfg(target_os = "windows")] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt macos => $expr:expr) => {
		#[cfg(target_os = "macos")] {
			break $cfg_table $expr;
		}
	};

	(@expand $cfg_table:tt linux32 => $expr:expr) => {
		#[cfg(all(target_os = "linux", target_pointer_width = "32"))] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt linux64 => $expr:expr) => {
		#[cfg(all(target_os = "linux", target_pointer_width = "64"))] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt win32 => $expr:expr) => {
		#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt win64 => $expr:expr) => {
		#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt macos32 => $expr:expr) => {
		#[cfg(all(target_os = "macos", target_pointer_width = "32"))] {
			break $cfg_table $expr;
		}
	};
	(@expand $cfg_table:tt macos64 => $expr:expr) => {
		#[cfg(all(target_os = "macos", target_pointer_width = "64"))] {
			break $cfg_table $expr;
		}
	};

	(@expand $cfg_table:tt [$cfg:meta] => $expr:expr) => {
		#[cfg($cfg)] {
			break $cfg_table $expr;
		}
	};

	{ _ => $default:expr, $($cfg:tt => $expr:expr),+ } => {
		'cfg_table: loop {
			#[allow(unreachable_code)] {
				$({ $crate::cfg_table!(@expand 'cfg_table $cfg => $expr) })+
				break 'cfg_table $default;
			}
		}
	};

	{ $($cfg:tt => $expr:expr),+ } => {
		'cfg_table: loop {
			#[allow(unreachable_code)] {
				$({ $crate::cfg_table!(@expand 'cfg_table $cfg => $expr) })+
			}
		}
	}
}

#[inline]
pub fn is_alive(ent: *mut std::ffi::c_void) -> bool {
	// string search for Alive, breakpoint + step into engine func
	#[allow(non_upper_case_globals)]
	const m_iLifeState: usize = cfg_table! {
		win64 => 352,
		win32 => 228,
		linux64 => 392,
		linux32 => 248
	};
	unsafe { *(((ent as usize) + m_iLifeState) as *mut u8) == 0 }
}