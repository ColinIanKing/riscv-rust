pub struct Clint {
	clock: u64,
	msip: u32,
	period_clock: u64,
	interrupting: bool
}

impl Clint {
	pub fn new() -> Self {
		Clint {
			clock: 0,
			msip: 0,
			period_clock: 0,
			interrupting: false
		}
	}

	pub fn tick(&mut self) {
		// @TODO: Implement more properly
		if (self.msip & 1) == 1 && self.period_clock > 0 && self.clock > self.period_clock {
			self.interrupting = true;
		}
		self.clock = self.clock.wrapping_add(1);
	}

	pub fn load(&self, address: u64) -> u8 {
		//println!("CLINT Load AD:{:X}", address);
		match address {
			// MSIP register 4 bytes
			0x02000000 => {
				(self.msip & 0xff) as u8
			},
			0x02000001 => {
				((self.msip >> 8) & 0xff) as u8
			},
			0x02000002 => {
				((self.msip >> 16) & 0xff) as u8
			},
			0x02000003 => {
				((self.msip >> 16) & 0xff) as u8
			},
			// MTIMECMP Registers 8 bytes
			0x02004000 => {
				(self.period_clock & 0xff) as u8
			},
			0x02004001 => {
				((self.period_clock >> 8) & 0xff) as u8
			},
			0x02004002 => {
				((self.period_clock >> 16) & 0xff) as u8
			},
			0x02004003 => {
				((self.period_clock >> 24) & 0xff) as u8
			},
			0x02004004 => {
				((self.period_clock >> 32) & 0xff) as u8
			},
			0x02004005 => {
				((self.period_clock >> 40) & 0xff) as u8
			},
			0x02004006 => {
				((self.period_clock >> 48) & 0xff) as u8
			},
			0x02004007 => {
				((self.period_clock >> 56) & 0xff) as u8
			},
			_ => 0,
		}
	}

	pub fn store(&mut self, address: u64, value: u8) {
		//println!("CLINT Store AD:{:X} VAL:{:X}", address, value);
		match address {
			// MSIP register 4 bytes
			0x02000000 => {
				self.msip = (self.msip & !0xff) | (value as u32);
			},
			0x02000001 => {
				self.msip = (self.msip & !0xff00) | ((value as u32) << 8);
			},
			0x02000002 => {
				self.msip = (self.msip & !0xff0000) | ((value as u32) << 16);
			},
			0x02000003 => {
				self.msip = (self.msip & !0xff000000) | ((value as u32) << 24);
			},
			// MTIMECMP Registers 8 bytes
			0x02004000 => {
				self.period_clock = (self.period_clock & !0xff) | (value as u64);
			},
			0x02004001 => {
				self.period_clock = (self.period_clock & !0xff00) | ((value as u64) << 8);
			},
			0x02004002 => {
				self.period_clock = (self.period_clock & !0xff0000) | ((value as u64) << 16);
			},
			0x02004003 => {
				self.period_clock = (self.period_clock & !0xff000000) | ((value as u64) << 24);
			},
			0x02004004 => {
				self.period_clock = (self.period_clock & !0xff00000000) | ((value as u64) << 32);
			},
			0x02004005 => {
				self.period_clock = (self.period_clock & !0xff0000000000) | ((value as u64) << 40);
			},
			0x02004006 => {
				self.period_clock = (self.period_clock & !0xff000000000000) | ((value as u64) << 48);
			},
			0x02004007 => {
				self.period_clock = (self.period_clock & !0xff00000000000000) | ((value as u64) << 56);
			},
			_ => {}
		};
	}

	pub fn is_interrupting(&self) -> bool {
		self.interrupting
	}

	pub fn reset_interrupting(&mut self) {
		self.interrupting = false;
		self.clock = 0;
	}
}
