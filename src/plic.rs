#[derive(Clone)]
pub enum InterruptType {
	None,
	KeyInput,
	Timer,
	TimerSoftware,
	Virtio
}

pub struct Plic {
	clock: u64,
	irq: u32,
	enabled: u64,
	threshold: u32,
	priorities: [u32; 1024],
	interrupt: InterruptType
}

impl Plic {
	pub fn new() -> Self {
		Plic {
			clock: 0,
			irq: 0,
			enabled: 0,
			threshold: 0,
			priorities: [0; 1024],
			interrupt: InterruptType::None
		}
	}

	pub fn tick(&mut self) {
		self.clock = self.clock.wrapping_add(1);
	}

	pub fn update(&mut self,
		virtio_is_interrupting: bool,
		uart_is_interrupting: bool,
		timer_is_interrupting: bool,
		software_timer_is_interrupting: bool) {

		let virtio_interrupt_id = 1;
		let uart_interrupt_id = 2;
		let timer_interrupt_id = 3;
		let timer_software_interrupt_id = 4;

		let virtio_irq = 1;
		let uart_irq = 10;

		// First detect external interrupts

		let virtio_priority = self.priorities[virtio_irq as usize];
		let uart_priority = self.priorities[uart_irq as usize];

		let virtio_enabled = ((self.enabled >> virtio_irq) & 1) == 1;
		let uart_enabled = ((self.enabled >> uart_irq) & 1) == 1;

		let interruptings = [virtio_is_interrupting, uart_is_interrupting];
		let enables = [virtio_enabled, uart_enabled];
		let priorities = [virtio_priority, uart_priority];

		let mut interrupt = 0;
		let mut priority = 0;
		for i in 0..2 {
			if interruptings[i] && enables[i] {
				if interrupt == 0 || (priorities[i] > priority) {
					interrupt = i + 1;
					priority = priorities[i];
				}
			}
		}

		if interrupt != 0 {
			if priority <= self.threshold {
				interrupt = 0;
			}
		}

		// Second, detect local interrupts if no external interrupts

		if interrupt == 0 {
			if timer_is_interrupting {
				interrupt = 3;
			} else if software_timer_is_interrupting {
				interrupt = 4;
			}
		}

		self.interrupt = match interrupt {
			1 => InterruptType::Virtio,
			2 => InterruptType::KeyInput,
			3 => InterruptType::Timer,
			4 => InterruptType::TimerSoftware,
			_ => InterruptType::None
		};

		let irq = match self.interrupt {
			InterruptType::Virtio => virtio_irq,
			InterruptType::KeyInput => uart_irq,
			_ => 0
		};

		if irq != 0 {
			self.irq = irq;
			//println!("IRQ: {:X}", self.irq);
		}
	}

	pub fn reset_interrupt(&mut self) {
		self.interrupt = InterruptType::None;
	}

	pub fn get_interrupt(&self) -> InterruptType {
		self.interrupt.clone()
	}

	pub fn load(&self, address: u64) -> u8 {
		//println!("PLIC Load AD:{:X}", address);
		match address {
			0x0c000000..=0x0c000ffc => {
				let offset = address % 4;
				let index = ((address - 0xc000000) >> 2) as usize;
				let pos = offset << 3;
				(self.priorities[index] >> pos) as u8
			},
			0x0c002080 => self.enabled as u8,
			0x0c002081 => (self.enabled >> 8) as u8,
			0x0c002082 => (self.enabled >> 16) as u8,
			0x0c002083 => (self.enabled >> 24) as u8,
			0x0c002084 => (self.enabled >> 32) as u8,
			0x0c002085 => (self.enabled >> 40) as u8,
			0x0c002086 => (self.enabled >> 48) as u8,
			0x0c002087 => (self.enabled >> 56) as u8,
			0x0c201000 => self.threshold as u8,
			0x0c201001 => (self.threshold >> 8) as u8,
			0x0c201002 => (self.threshold >> 16) as u8,
			0x0c201003 => (self.threshold >> 24) as u8,
			0x0c201004 => {
				//println!("PLIC IRQ:{:X}", self.irq);
				self.irq as u8
			},
			0x0c201005 => (self.irq >> 8) as u8,
			0x0c201006 => (self.irq >> 16) as u8,
			0x0c201007 => (self.irq >> 24) as u8,
			_ => 0
		}
	}

	pub fn store(&mut self, address: u64, value: u8) {
		//println!("PLIC Store AD:{:X} VAL:{:X}", address, value);
		match address {
			0x0c000000..=0x0c000ffc => {
				let offset = address % 4;
				let index = ((address - 0xc000000) >> 2) as usize;
				let pos = offset << 3;
				self.priorities[index] = (self.priorities[index] & !(0xff << pos)) | ((value as u32) << pos);
			},
			0x0c002080 => {
				self.enabled = (self.enabled & !0xff) | (value as u64);
			},
			0x0c002081 => {
				self.enabled = (self.enabled & !0xff00) | ((value as u64) << 8);
			},
			0x0c002082 => {
				self.enabled = (self.enabled & !0xff0000) | ((value as u64) << 16);
			},
			0x0c002083 => {
				self.enabled = (self.enabled & !0xff000000) | ((value as u64) << 24);
			},
			0x0c002084 => {
				self.enabled = (self.enabled & !0xff00000000) | ((value as u64) << 32);
			},
			0x0c002085 => {
				self.enabled = (self.enabled & !0xff0000000000) | ((value as u64) << 40);
			},
			0x0c002086 => {
				self.enabled = (self.enabled & !0xff000000000000) | ((value as u64) << 48);
			},
			0x0c002087 => {
				self.enabled = (self.enabled & !0xff00000000000000) | ((value as u64) << 56);
			},
			0x0c201000 => {
				self.threshold = (self.threshold & !0xff) | (value as u32);
			},
			0x0c201001 => {
				self.threshold = (self.threshold & !0xff00) | ((value as u32) << 8);
			},
			0x0c201002 => {
				self.threshold = (self.threshold & !0xff0000) | ((value as u32) << 16);
			},
			0x0c201003 => {
				self.threshold = (self.threshold & !0xff000000) | ((value as u32) << 24);
			},
			0x0c201004 => {
				if self.irq as u8 == value {
					self.irq = 0;
				}
			},
			_ => {}
		};
	}
}
