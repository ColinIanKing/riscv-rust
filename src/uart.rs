use terminal::Terminal;

pub struct Uart {
	clock: u64,
	receive_register: u8,
	line_status_register: u8,
	interrupt_enable_register: u8,
	interrupt_identification_register: u8,
	line_control_register: u8,
	interrupting: bool,
	terminal: Box<dyn Terminal>
}

impl Uart {
	pub fn new(terminal: Box<dyn Terminal>) -> Self {
		Uart {
			clock: 0,
			receive_register: 0,
			line_status_register: 0x20,
			interrupt_enable_register: 0,
			interrupt_identification_register: 0xf,
			line_control_register: 0,
			interrupting: false,
			terminal: terminal
		}
	}

	pub fn tick(&mut self) {
		self.clock = self.clock.wrapping_add(1);
		if (self.clock % 0x10000) == 0 && !self.interrupting { // @TODO: Fix me
			let value = self.terminal.get_input();
			if value != 0 {
				if (self.interrupt_enable_register & 1) != 0 {
					self.interrupting = true;
					self.interrupt_identification_register = 0x4;
				}
				self.receive_register = value;
				self.line_status_register = 0x21; // @TODO: Is this correct?
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
			// Receiver Buffer Register
			0x10000000 => match (self.line_control_register >> 7) == 0 {
				true => {
					if (self.interrupt_identification_register & 0xe) == 0x4 {
						self.interrupt_identification_register = 0xf;
					}
					let value = self.receive_register;
					self.receive_register = 0x0;
					self.line_status_register = 0x20;
					value
				},
				false => 0 // @TODO: Implement properly
			},
			0x10000001 => match (self.line_control_register >> 7) == 0 {
				true => self.interrupt_enable_register,
				false => 0 // @TODO: Implement properly
			},
			0x10000002 => match (self.line_control_register >> 7) == 0 {
				true => {
					if (self.interrupt_identification_register & 0xe) != 0x2 {
						self.interrupt_identification_register = 0xf;
					}
					self.interrupt_identification_register
				},
				false => 0 // @TODO: Implement properly
			},
			0x10000003 => self.line_control_register,
			0x10000005 => match (self.line_control_register >> 7) == 0 {
				true => self.line_status_register, // UART0 LSR
				false => 0 // @TODO: Implement properly
			},
			_ => 0
		}
	}

	pub fn store(&mut self, address: u64, value: u8) {
		//println!("UART Store AD:{:X} VAL:{:X}", address, value);
		match address {
			// Transfer Holding Register
			0x10000000 => match (self.line_control_register >> 7) == 0 { // UART0 THR
				true => {
					self.terminal.put_byte(value);
					if (self.interrupt_enable_register & 2) != 0 {
						self.interrupting = true;
						self.interrupt_identification_register = 0x3;
					} else if (self.interrupt_identification_register & 0xe) != 0x2 {
						self.interrupt_identification_register = 0xf;
					}
				},
				false => {} // @TODO: Implement properly
			},
			0x10000001 => match (self.line_control_register >> 7) == 0 {
				true => {
					self.interrupt_enable_register = value;
				},
				false => {} // @TODO: Implement properly
			},
			0x10000003 => {
				self.line_control_register = value;
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
