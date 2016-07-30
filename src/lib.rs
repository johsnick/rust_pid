extern crate time;

pub struct PID {
	p_const: i16,
	i_const: i16,
	d_const: i16,
	updated_at: time::Timespec,
	integral: i16,
	last_input: i16,
	max_output: i16,
}

pub enum PIDConst {
	P(i16),
	I(i16),
	D(i16),
}

impl PID {
	pub fn new(p_const: i16, i_const: i16, d_const: i16, max_output: i16) -> PID {
		PID {
			p_const: p_const,
			i_const: i_const,
			d_const: d_const,
			updated_at: time::get_time(),
			integral: 0,
			last_input: 0,
			max_output: max_output,
		}
	}

	pub fn compute(&mut self, error: i16) -> i16 {
		let p_error = self.p_const * error;

		let i_error = {
			let time_since_last = (time::get_time() - self.updated_at).num_milliseconds();
			self.updated_at = time::get_time();
			self.integral += (time_since_last as i16) * error * self.i_const;
			self.integral
		};

		let d_error = (error - (self.last_input as i16)) * self.d_const; 

		self.last_input = error;

		let output = p_error + i_error - d_error;

		if output > self.max_output {
			self.max_output
		} else {
			output
		}
	}

	pub fn update_const(&mut self, value: PIDConst){
		match value {
			PIDConst::P(p) => self.p_const = p,
			PIDConst::I(i) => self.i_const = i,
			PIDConst::D(d) => self.d_const = d,
		}
	}

	pub fn udate_consts(&mut self, p_const: i16, i_const: i16, d_const: i16) {
		self.p_const = p_const;
		self.i_const = i_const;
		self.d_const = d_const;
	}
}