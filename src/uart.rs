use terminal::Terminal;

pub struct Uart {
	clock: u64,
	rbr: u8, // receiver buffer register
	ier: u8, // interrupt enable register
	iir: u8, // interrupt identification register
	lcr: u8, // line control register
	mcr: u8, // modem control register
	lsr: u8, // line status register
	scr: u8, // scratch
	interrupting: bool,
	terminal: Box<dyn Terminal>
}

impl Uart {
	pub fn new(terminal: Box<dyn Terminal>) -> Self {
		Uart {
			clock: 0,
			rbr: 0,
			ier: 0,
			iir: 0x02,
			lcr: 0,
			mcr: 0,
			lsr: 0x20,
			scr: 0,
			interrupting: false,
			terminal: terminal
		}
	}

	pub fn tick(&mut self) {
		self.clock = self.clock.wrapping_add(1);
		if (self.clock % 0x384000) == 0 && !self.interrupting { // @TODO: Fix me
			let value = self.terminal.get_input();
			if value != 0 {
				if (self.ier & 0x1) != 0 {
					self.interrupting = true;
					self.iir = 0x04;
				}
				self.rbr = value;
				self.lsr |= 0x01;
			} else {
				if (self.ier & 0x2) != 0 {
					self.interrupting = true;
					self.iir = 0x02;
				}
				self.lsr |= 0x20;
			}
		}
	}

	pub fn is_interrupting(&self) -> bool {
		self.interrupting
	}

	pub fn reset_interrupting(&mut self) {
		self.interrupting = false;
	}

	pub fn load(&mut self, address: u64) -> u8 {
		//println!("UART Load AD:{:X}", address);
		match address {
			0x10000000 => match (self.lcr >> 7) == 0 {
				true => {
					if (self.iir & 0x0e) == 0x04 {
						self.iir |= 0x0e;
					}
					let rbr = self.rbr;
					self.rbr = 0;
					self.lsr &= !0x01;
					rbr
				},
				false => 0 // @TODO: Implement properly
			},
			0x10000001 => match (self.lcr >> 7) == 0 {
				true => self.ier,
				false => 0 // @TODO: Implement properly
			},
			0x10000002 => {
				let iir = self.iir;
				if (self.iir & 0x0e) == 0x02 {
					self.iir |= 0x0e;
				}
				self.iir |= 0x1; // Necessary?
				iir
			},
			0x10000003 => self.lcr,
			0x10000004 => self.mcr,
			0x10000005 => self.lsr,
			0x10000007 => self.scr,
			_ => 0
		}
	}

	pub fn store(&mut self, address: u64, value: u8) {
		//println!("UART Store AD:{:X} VAL:{:X}", address, value);
		match address {
			// Transfer Holding Register
			0x10000000 => match (self.lcr >> 7) == 0 {
				true => {
					self.terminal.put_byte(value);
					if (!self.interrupting) {
						if (self.ier & 2) != 0 {
							self.interrupting = true;
							self.iir = 0x2;
						}
					}
					self.lsr |= 0x20;
				},
				false => {} // @TODO: Implement properly
			},
			0x10000001 => match (self.lcr >> 7) == 0 {
				true => {
					self.ier = value;
				},
				false => {} // @TODO: Implement properly
			},
			0x10000003 => {
				self.lcr = value;
			},
			0x10000004 => {
				self.mcr = value;
			},
			0x10000007 => {
				self.scr = value;
			},
			_ => {}
		};
	}

	// Wasm specific

	pub fn get_output(&mut self) -> u8 {
		self.terminal.get_output()
	}

	pub fn put_output(&mut self, data: u8) {
		self.terminal.put_byte(data);
	}

	pub fn put_input(&mut self, data: u8) {
		self.terminal.put_input(data);
	}
}
