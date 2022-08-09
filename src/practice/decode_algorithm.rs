
struct Decode<'a> {
    buffer: &'a mut [i32],
    coefficients: [i64; 12],
    qlp_shift: i16
}

impl<'a> Decode<'a> {
    fn decode(self) {
        for i in 12..(self.buffer).len() {
            let prediction = self.coefficients.iter().zip(&self.buffer[i - 12..i]).map(|(&c, &b)| c * b as i64).sum::<i64>() >> self.qlp_shift;
            let delta = self.buffer[i];
            self.buffer[i] = prediction as i32 + delta;
        }
    }
}


fn decode_alg() {

}