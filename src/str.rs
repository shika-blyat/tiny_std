pub struct String {
    vec: Vec<u8>,
}
impl String{
	pub fn as_bytes(&self) -> &[u8]{
		&self.vec
	}
	pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
        String { vec: bytes }
    }
}
