/// Module containing identifiers and byte sizes for parsing an IcbiaDB bin-file
///
pub mod kv;
pub mod decl;


pub mod globals {

	pub const USIZE_BS: usize = std::mem::size_of::<usize>();
	pub const U16_BS: usize = std::mem::size_of::<u16>();
	pub const U32_BS: usize = std::mem::size_of::<u32>();
	pub const U64_BS: usize = std::mem::size_of::<u64>();

	pub const K_LEN_BS: usize = 1; // Key length byte size
	pub const TN_LEN_BS: usize = 1; // Type name length byte size
	pub const V_LEN_BS: usize = U32_BS; // Value length byte size

	/// Module of declaration byte sizes
	pub mod decl {
		use super::*;

		pub const LEN_BS: usize = U32_BS; // Byte size of all declarations total length

		/// Declaration identifier
		pub const IDENT: [u8; 3] = [0x0, 0x1E, 120]; // \x00x
		pub const IDENT_HEAD_BS: usize = IDENT.len() + K_LEN_BS + U16_BS;

		/// Module of declaration records byte sizes
		pub mod records {
			use super::*;

			/// Declaration records identifier
			pub const IDENT: [u8; 3] = [0x2, 0x1E, 120];	// \x02x
			pub const RECORD_IDENT: [u8; 3] = [3, 0x1E, 120];	// \x02x


			/// Declaration single record identifier 
			pub const IDENT_HEAD_BS: usize = IDENT.len() + RECORDS_LEN_BS;
			pub const RECORD_IDENT_HEAD_BS: usize = RECORD_IDENT.len() + RECORD_LEN_BS;

			pub const RECORDS_LEN_BS: usize = U64_BS; // Byte size of a declaration records total length
			pub const RECORD_LEN_BS: usize = U32_BS; // Byte size of a single records total length
		}

	}

	pub mod kv {
		/// Module of Key-Value byte sizes
		use super::*;

		pub const TOTAL_LEN_BS: usize = U64_BS; // Byte size of a KV records total length

		/// KV record identifier
		pub const IDENT: [u8; 3] = [0x1, 0x1E, 120]; // \x01x
		pub const IDENT_HEAD_BS: usize = IDENT.len() + K_LEN_BS + TN_LEN_BS + V_LEN_BS;
	}

}





