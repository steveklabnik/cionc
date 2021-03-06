use std::cell::RefCell;
use std::ops::DerefMut;
use super::string_table::StringTable;

// This type is the root to manage several subcomponents.
// In later revisions it has a StringTable, a SymbolTable, an ErrorHandler and more
// important compile modules that other compile subcomponents like the Lexer or Parser
// can use.
// It has methods to get references to its members so types like Lexer and Parser just
// have to have one member of this type in order to access all of its utility members.
//
// However, as it seems to me at the moment this design doesn't suite the borrow-checker
// and especially the lifetime-checker of Rust and may be a bad design in general.
// I am very happy about suggestions to improve this situation as I am unwilling to just use
// unsafe blocks where they aren't needed for sure.

#[derive(Default)]
pub struct CompileContext {
	string_table: RefCell<StringTable>
}

impl CompileContext {
	// This method doesn't work, the compiler complains about the following:
	// src/parser/compile_context.rs:24:3: 24:33 error: borrowed value does not live long enough
	// src/parser/compile_context.rs:24 		self.string_table.borrow_mut().deref_mut()
	// src/parser/compile_context.rs:23:57: 25:3 note: reference must be valid for the anonymous lifetime #1 defined on the block at 23:56...
	// src/parser/compile_context.rs:23:57: 25:3 note: ...but borrowed value is only valid for the block at 23:56
	//
	// I think a big problem I still have is that I haven't yet learned how to precisely
	// express my intents to the lifetime-checker.
	//
	// pub fn get_string_table(&mut self) -> &mut StringTable {
	// 	self.string_table.borrow_mut().deref_mut()
	// }
}