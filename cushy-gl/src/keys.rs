

///////////////////////////////////////////////////////////////////////////////////////////////////// Key

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
	// Printable keys
	Space,			Apostrophe,		Comma,			Minus,			Period,			Slash,
	Num0,			Num1,			Num2,			Num3,			Num4,			Num5,
	Num6,			Num7,			Num8,			Num9,			Semicolon,		Equal,
	A,				B,				C,				D,				E,				F,
	G,				H,				I,				J,				K,				L,
	M,				N,				O,				P,				Q,				R,
	S,				T,				U,				V,				W,				X,
	Y,				Z,				LeftBracket,	Backslash,		RightBracket,	GraveAccent,

	// Non-printable keys
	World1,			World2,			Escape,			Enter,			Tab,			Backspace,
	Insert,			Delete,			Right,			Left,			Down,			Up,
	PageUp,			PageDown,		Home,			End,			CapsLock,		ScrollLock,
	NumLock,		PrintScreen,	Pause,			F1,				F2,				F3,
	F4,				F5,				F6,				F7,				F8,				F9,
	F10,			F11,			F12,			F13,			F14,			F15,
	F16,			F17,			F18,			F19,			F20,			F21,
	F22,			F23,			F24,			F25,			Kp0,			Kp1,
	Kp2,			Kp3,			Kp4,			Kp5,			Kp6,			Kp7,
	Kp8,			Kp9,			KpDecimal,		KpDivide,		KpMultiply,		KpSubtract,
	KpAdd,			KpEnter,		KpEqual,		LeftShift,		LeftControl,	LeftAlt,
	LeftWin,		RightShift,		RightControl,	RightAlt,		RightWin,		Menu,
}

impl Key {

	pub fn is_printable(&self) -> bool {
		*self<Key::World1
	}

	pub fn display_name(&self) -> &'static str {
		// Get the key name, meant to be shown to the user
		match *self {
			Key::Space			=> "Space",
			Key::Apostrophe		=> "'",
			Key::Comma			=> ",",
			Key::Minus			=> "-",
			Key::Period			=> ".",
			Key::Slash			=> "/",
			Key::Num0			=> "0",
			Key::Num1			=> "1",
			Key::Num2			=> "2",
			Key::Num3			=> "3",
			Key::Num4			=> "4",
			Key::Num5			=> "5",
			Key::Num6			=> "6",
			Key::Num7			=> "7",
			Key::Num8			=> "8",
			Key::Num9			=> "9",
			Key::Semicolon		=> ";",
			Key::Equal			=> "=",
			Key::A				=> "A",
			Key::B				=> "B",
			Key::C				=> "C",
			Key::D				=> "D",
			Key::E				=> "E",
			Key::F				=> "F",
			Key::G				=> "G",
			Key::H				=> "H",
			Key::I				=> "I",
			Key::J				=> "J",
			Key::K				=> "K",
			Key::L				=> "L",
			Key::M				=> "M",
			Key::N				=> "N",
			Key::O				=> "O",
			Key::P				=> "P",
			Key::Q				=> "Q",
			Key::R				=> "R",
			Key::S				=> "S",
			Key::T				=> "T",
			Key::U				=> "U",
			Key::V				=> "V",
			Key::W				=> "W",
			Key::X				=> "X",
			Key::Y				=> "Y",
			Key::Z				=> "Z",
			Key::LeftBracket	=> "[",
			Key::Backslash		=> "\\",
			Key::RightBracket	=> "]",
			Key::GraveAccent	=> "`",
			Key::World1			=> "World1",
			Key::World2			=> "World2",
			Key::Escape			=> "Esc",
			Key::Enter			=> "Enter",
			Key::Tab			=> "Tab",
			Key::Backspace		=> "Backspace",
			Key::Insert			=> "Insert",
			Key::Delete			=> "Delete",
			Key::Right			=> "→",
			Key::Left			=> "←",
			Key::Down			=> "↓",
			Key::Up				=> "↑",
			Key::PageUp			=> "Page Up",
			Key::PageDown		=> "Page Down",
			Key::Home			=> "Home",
			Key::End			=> "End",
			Key::CapsLock		=> "Caps Lock",
			Key::ScrollLock		=> "Scroll Lock",
			Key::NumLock		=> "Num Lock",
			Key::PrintScreen	=> "Print Screen",
			Key::Pause			=> "Pause",
			Key::F1				=> "F1",
			Key::F2				=> "F2",
			Key::F3				=> "F3",
			Key::F4				=> "F4",
			Key::F5				=> "F5",
			Key::F6				=> "F6",
			Key::F7				=> "F7",
			Key::F8				=> "F8",
			Key::F9				=> "F9",
			Key::F10			=> "F10",
			Key::F11			=> "F11",
			Key::F12			=> "F12",
			Key::F13			=> "F13",
			Key::F14			=> "F14",
			Key::F15			=> "F15",
			Key::F16			=> "F16",
			Key::F17			=> "F17",
			Key::F18			=> "F18",
			Key::F19			=> "F19",
			Key::F20			=> "F20",
			Key::F21			=> "F21",
			Key::F22			=> "F22",
			Key::F23			=> "F23",
			Key::F24			=> "F24",
			Key::F25			=> "F25",
			Key::Kp0			=> "Keypad 0",
			Key::Kp1			=> "Keypad 1",
			Key::Kp2			=> "Keypad 2",
			Key::Kp3			=> "Keypad 3",
			Key::Kp4			=> "Keypad 4",
			Key::Kp5			=> "Keypad 5",
			Key::Kp6			=> "Keypad 6",
			Key::Kp7			=> "Keypad 7",
			Key::Kp8			=> "Keypad 8",
			Key::Kp9			=> "Keypad 9",
			Key::KpDecimal		=> "Keypad .",
			Key::KpDivide		=> "Keypad /",
			Key::KpMultiply		=> "Keypad *",
			Key::KpSubtract		=> "Keypad -",
			Key::KpAdd			=> "Keypad +",
			Key::KpEnter		=> "Keypad Enter",
			Key::KpEqual		=> "Keypad =",
			Key::LeftShift		=> "Left Shift",
			Key::LeftControl	=> "Left Control",
			Key::LeftAlt		=> "Left Alt",
			Key::LeftWin		=> "Left Windows",
			Key::RightShift		=> "Right Shift",
			Key::RightControl	=> "Right Control",
			Key::RightAlt		=> "Right Alt",
			Key::RightWin		=> "Right Windows",
			Key::Menu			=> "Menu",
		}
	}

	pub fn sym_name(&self) -> &'static str {
		// Get the symbol name for the key, same as enum name
		match *self {
			Key::Space			=> "Space",
			Key::Apostrophe		=> "Apostrophe",
			Key::Comma			=> "Comma",
			Key::Minus			=> "Minus",
			Key::Period			=> "Period",
			Key::Slash			=> "Slash",
			Key::Num0			=> "Num0",
			Key::Num1			=> "Num1",
			Key::Num2			=> "Num2",
			Key::Num3			=> "Num3",
			Key::Num4			=> "Num4",
			Key::Num5			=> "Num5",
			Key::Num6			=> "Num6",
			Key::Num7			=> "Num7",
			Key::Num8			=> "Num8",
			Key::Num9			=> "Num9",
			Key::Semicolon		=> "Semicolon",
			Key::Equal			=> "Equal",
			Key::A				=> "A",
			Key::B				=> "B",
			Key::C				=> "C",
			Key::D				=> "D",
			Key::E				=> "E",
			Key::F				=> "F",
			Key::G				=> "G",
			Key::H				=> "H",
			Key::I				=> "I",
			Key::J				=> "J",
			Key::K				=> "K",
			Key::L				=> "L",
			Key::M				=> "M",
			Key::N				=> "N",
			Key::O				=> "O",
			Key::P				=> "P",
			Key::Q				=> "Q",
			Key::R				=> "R",
			Key::S				=> "S",
			Key::T				=> "T",
			Key::U				=> "U",
			Key::V				=> "V",
			Key::W				=> "W",
			Key::X				=> "X",
			Key::Y				=> "Y",
			Key::Z				=> "Z",
			Key::LeftBracket	=> "LeftBracket",
			Key::Backslash		=> "Backslash",
			Key::RightBracket	=> "RightBracket",
			Key::GraveAccent	=> "GraveAccent",
			Key::World1			=> "World1",
			Key::World2			=> "World2",
			Key::Escape			=> "Escape",
			Key::Enter			=> "Enter",
			Key::Tab			=> "Tab",
			Key::Backspace		=> "Backspace",
			Key::Insert			=> "Insert",
			Key::Delete			=> "Delete",
			Key::Right			=> "Right",
			Key::Left			=> "Left",
			Key::Down			=> "Down",
			Key::Up				=> "Up",
			Key::PageUp			=> "PageUp",
			Key::PageDown		=> "PageDown",
			Key::Home			=> "Home",
			Key::End			=> "End",
			Key::CapsLock		=> "CapsLock",
			Key::ScrollLock		=> "ScrollLock",
			Key::NumLock		=> "NumLock",
			Key::PrintScreen	=> "PrintScreen",
			Key::Pause			=> "Pause",
			Key::F1				=> "F1",
			Key::F2				=> "F2",
			Key::F3				=> "F3",
			Key::F4				=> "F4",
			Key::F5				=> "F5",
			Key::F6				=> "F6",
			Key::F7				=> "F7",
			Key::F8				=> "F8",
			Key::F9				=> "F9",
			Key::F10			=> "F10",
			Key::F11			=> "F11",
			Key::F12			=> "F12",
			Key::F13			=> "F13",
			Key::F14			=> "F14",
			Key::F15			=> "F15",
			Key::F16			=> "F16",
			Key::F17			=> "F17",
			Key::F18			=> "F18",
			Key::F19			=> "F19",
			Key::F20			=> "F20",
			Key::F21			=> "F21",
			Key::F22			=> "F22",
			Key::F23			=> "F23",
			Key::F24			=> "F24",
			Key::F25			=> "F25",
			Key::Kp0			=> "Kp0",
			Key::Kp1			=> "Kp1",
			Key::Kp2			=> "Kp2",
			Key::Kp3			=> "Kp3",
			Key::Kp4			=> "Kp4",
			Key::Kp5			=> "Kp5",
			Key::Kp6			=> "Kp6",
			Key::Kp7			=> "Kp7",
			Key::Kp8			=> "Kp8",
			Key::Kp9			=> "Kp9",
			Key::KpDecimal		=> "KpDecimal",
			Key::KpDivide		=> "KpDivide",
			Key::KpMultiply		=> "KpMultiply",
			Key::KpSubtract		=> "KpSubtract",
			Key::KpAdd			=> "KpAdd",
			Key::KpEnter		=> "KpEnter",
			Key::KpEqual		=> "KpEqual",
			Key::LeftShift		=> "LeftShift",
			Key::LeftControl	=> "LeftControl",
			Key::LeftAlt		=> "LeftAlt",
			Key::LeftWin		=> "LeftWin",
			Key::RightShift		=> "RightShift",
			Key::RightControl	=> "RightControl",
			Key::RightAlt		=> "RightAlt",
			Key::RightWin		=> "RightWin",
			Key::Menu			=> "Menu",
		}
	}


	pub fn from_glfw(gkey: glfw::Key) -> Key {
		// Create the GLFW->JMGE key map
		match gkey {
			glfw::Key::Space			=> Key::Space,
			glfw::Key::Apostrophe		=> Key::Apostrophe,
			glfw::Key::Comma			=> Key::Comma,
			glfw::Key::Minus			=> Key::Minus,
			glfw::Key::Period			=> Key::Period,
			glfw::Key::Slash			=> Key::Slash,
			glfw::Key::Num0				=> Key::Num0,
			glfw::Key::Num1				=> Key::Num1,
			glfw::Key::Num2				=> Key::Num2,
			glfw::Key::Num3				=> Key::Num3,
			glfw::Key::Num4				=> Key::Num4,
			glfw::Key::Num5				=> Key::Num5,
			glfw::Key::Num6				=> Key::Num6,
			glfw::Key::Num7				=> Key::Num7,
			glfw::Key::Num8				=> Key::Num8,
			glfw::Key::Num9				=> Key::Num9,
			glfw::Key::Semicolon		=> Key::Semicolon,
			glfw::Key::Equal			=> Key::Equal,
			glfw::Key::A				=> Key::A,
			glfw::Key::B				=> Key::B,
			glfw::Key::C				=> Key::C,
			glfw::Key::D				=> Key::D,
			glfw::Key::E				=> Key::E,
			glfw::Key::F				=> Key::F,
			glfw::Key::G				=> Key::G,
			glfw::Key::H				=> Key::H,
			glfw::Key::I				=> Key::I,
			glfw::Key::J				=> Key::J,
			glfw::Key::K				=> Key::K,
			glfw::Key::L				=> Key::L,
			glfw::Key::M				=> Key::M,
			glfw::Key::N				=> Key::N,
			glfw::Key::O				=> Key::O,
			glfw::Key::P				=> Key::P,
			glfw::Key::Q				=> Key::Q,
			glfw::Key::R				=> Key::R,
			glfw::Key::S				=> Key::S,
			glfw::Key::T				=> Key::T,
			glfw::Key::U				=> Key::U,
			glfw::Key::V				=> Key::V,
			glfw::Key::W				=> Key::W,
			glfw::Key::X				=> Key::X,
			glfw::Key::Y				=> Key::Y,
			glfw::Key::Z				=> Key::Z,
			glfw::Key::LeftBracket		=> Key::LeftBracket,
			glfw::Key::Backslash		=> Key::Backslash,
			glfw::Key::RightBracket		=> Key::RightBracket,
			glfw::Key::GraveAccent		=> Key::GraveAccent,
			glfw::Key::World1			=> Key::World1,
			glfw::Key::World2			=> Key::World2,
			glfw::Key::Escape			=> Key::Escape,
			glfw::Key::Enter			=> Key::Enter,
			glfw::Key::Tab				=> Key::Tab,
			glfw::Key::Backspace		=> Key::Backspace,
			glfw::Key::Insert			=> Key::Insert,
			glfw::Key::Delete			=> Key::Delete,
			glfw::Key::Right			=> Key::Right,
			glfw::Key::Left				=> Key::Left,
			glfw::Key::Down				=> Key::Down,
			glfw::Key::Up				=> Key::Up,
			glfw::Key::PageUp			=> Key::PageUp,
			glfw::Key::PageDown			=> Key::PageDown,
			glfw::Key::Home				=> Key::Home,
			glfw::Key::End				=> Key::End,
			glfw::Key::CapsLock			=> Key::CapsLock,
			glfw::Key::ScrollLock		=> Key::ScrollLock,
			glfw::Key::NumLock			=> Key::NumLock,
			glfw::Key::PrintScreen		=> Key::PrintScreen,
			glfw::Key::Pause			=> Key::Pause,
			glfw::Key::F1				=> Key::F1,
			glfw::Key::F2				=> Key::F2,
			glfw::Key::F3				=> Key::F3,
			glfw::Key::F4				=> Key::F4,
			glfw::Key::F5				=> Key::F5,
			glfw::Key::F6				=> Key::F6,
			glfw::Key::F7				=> Key::F7,
			glfw::Key::F8				=> Key::F8,
			glfw::Key::F9				=> Key::F9,
			glfw::Key::F10				=> Key::F10,
			glfw::Key::F11				=> Key::F11,
			glfw::Key::F12				=> Key::F12,
			glfw::Key::F13				=> Key::F13,
			glfw::Key::F14				=> Key::F14,
			glfw::Key::F15				=> Key::F15,
			glfw::Key::F16				=> Key::F16,
			glfw::Key::F17				=> Key::F17,
			glfw::Key::F18				=> Key::F18,
			glfw::Key::F19				=> Key::F19,
			glfw::Key::F20				=> Key::F20,
			glfw::Key::F21				=> Key::F21,
			glfw::Key::F22				=> Key::F22,
			glfw::Key::F23				=> Key::F23,
			glfw::Key::F24				=> Key::F24,
			glfw::Key::F25				=> Key::F25,
			glfw::Key::Kp0				=> Key::Kp0,
			glfw::Key::Kp1				=> Key::Kp1,
			glfw::Key::Kp2				=> Key::Kp2,
			glfw::Key::Kp3				=> Key::Kp3,
			glfw::Key::Kp4				=> Key::Kp4,
			glfw::Key::Kp5				=> Key::Kp5,
			glfw::Key::Kp6				=> Key::Kp6,
			glfw::Key::Kp7				=> Key::Kp7,
			glfw::Key::Kp8				=> Key::Kp8,
			glfw::Key::Kp9				=> Key::Kp9,
			glfw::Key::KpDecimal		=> Key::KpDecimal,
			glfw::Key::KpDivide			=> Key::KpDivide,
			glfw::Key::KpMultiply		=> Key::KpMultiply,
			glfw::Key::KpSubtract		=> Key::KpSubtract,
			glfw::Key::KpAdd			=> Key::KpAdd,
			glfw::Key::KpEnter			=> Key::KpEnter,
			glfw::Key::KpEqual			=> Key::KpEqual,
			glfw::Key::LeftShift		=> Key::LeftShift,
			glfw::Key::LeftControl		=> Key::LeftControl,
			glfw::Key::LeftAlt			=> Key::LeftAlt,
			glfw::Key::LeftSuper		=> Key::LeftWin,
			glfw::Key::RightShift		=> Key::RightShift,
			glfw::Key::RightControl		=> Key::RightControl,
			glfw::Key::RightAlt			=> Key::RightAlt,
			glfw::Key::RightSuper		=> Key::RightWin,
			glfw::Key::Menu				=> Key::Menu,
			glfw::Key::Unknown			=> unreachable!(),
		}
	}

}


///////////////////////////////////////////////////////////////////////////////////////////////////// Modifiers

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Modifiers {
	pub shift: bool,
	pub control: bool,
	pub alt: bool,
	pub win: bool,
}

impl Modifiers {
	
	pub fn new() -> Modifiers {
		Modifiers {
			shift: false,
			control: false,
			alt: false,
			win: false,
		}
	}

	pub fn with(shift:bool, control:bool, alt:bool, win:bool) -> Modifiers {
		Modifiers {
			shift,
			control,
			alt,
			win
		}
	}

	pub fn none() -> Modifiers				{ Modifiers::new() }
	pub fn shift() -> Modifiers				{ Modifiers::with(true, false, false, false) }
	pub fn control() -> Modifiers			{ Modifiers::with(false, true, false, false) }
	pub fn alt() -> Modifiers				{ Modifiers::with(false, false, true, false) }
	pub fn win() -> Modifiers				{ Modifiers::with(false, false, false, true) }

	pub fn ctrl_alt() -> Modifiers			{ Modifiers::with(false, true, true, false) }
	pub fn ctrl_shift() -> Modifiers		{ Modifiers::with(true, true, false, false) }
	pub fn alt_shift() -> Modifiers			{ Modifiers::with(true, false, true, false) }

	pub fn from_glfw(mods: glfw::Modifiers) -> Modifiers {
		// Convert GLFW modifiers
		Modifiers {
			shift: mods.intersects(glfw::Modifiers::Shift),
			control: mods.intersects(glfw::Modifiers::Control),
			alt: mods.intersects(glfw::Modifiers::Alt),
			win: mods.intersects(glfw::Modifiers::Super),
		}
	}
	
	pub fn to_string_alt(&self) -> String {
		let mut s: String = String::new();
		
		if self.shift
			{ s.push('⇧'); }
		if self.control
			{ s.push('^'); }
		if self.alt
			{ s.push('!'); }
		if self.win
			{ s.push('#'); }

		s
	}

}

impl ToString for Modifiers
{
	fn to_string(&self) -> String {
		let mut s: String = String::new();
		
		if self.shift
			{ s.push('S'); }
		if self.control
			{ s.push('C'); }
		if self.alt
			{ s.push('A'); }
		if self.win
			{ s.push('W'); }

		s
	}
}
