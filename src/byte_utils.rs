use std::ops;


#[derive(Debug, PartialEq, Clone)]
pub struct Bytes {
    bytes: Vec<u8>
}

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Bytes {
        return Bytes { bytes }
    }

    fn _binary_sub(x: &u8, y: &u8, init_brw: u8) -> (u8, u8) {
        let mut res = 0b0;
        let mut brw = init_brw;

        for shf in 0..8 {
            let ibit = 0b1 << shf;
            let ibrw = brw << shf;
            let ybit = y & ibit;
            let xbit = x & ibit;
            let bit = xbit ^ ybit ^ ibrw;
            res |= bit;

            brw = bit & (ybit | ibrw);
            brw >>= shf;
            brw &= 0b1;
        }

        return (res, brw);
    }

    fn _binary_add(x: &u8, y: &u8, init_carry: u8) -> (u8, u8) {
        let mut res = 0b0;
        let mut car = init_carry;

        for shf in 0..8 {
            let ibit = 0b1 << shf;
            let icar = car << shf;
            let ybit = y & ibit;
            let xbit = x & ibit;
            let bit = ybit ^ xbit ^ icar;
            res |= bit;
            car = (ybit & xbit) | (xbit & icar) | (ybit & icar);
            car >>= shf;
            car &= 0b1;
        }

        return (res, car);
    }

    pub fn is_equal(&self, rhs: &Bytes) -> bool {
        let longest: &Bytes;
        let shortest: &Bytes;

        if self.bytes.len() > rhs.bytes.len() {
            longest = self;
            shortest = rhs;
        } else {
            longest = rhs;
            shortest = self;
        }

        for i in 0..longest.bytes.len() {
            let lhs_byte = longest.bytes.get(i).unwrap_or(&0b0);
            let rhs_byte = shortest.bytes.get(i).unwrap_or(&0b0);

            if lhs_byte != rhs_byte {
                return false
            }
        }

        return true;
    }

    pub fn pow(&self, exponent: Bytes) -> Bytes {
        let mut res = Bytes::new(vec![1]);
        let mut exp_copy = exponent;       


        while !exp_copy.is_equal(&Bytes::new(vec![0])) {
            res = res * Bytes::new(self.bytes.clone());
            exp_copy = exp_copy - Bytes::new(vec![1]);
        }

        return res
    }
}

impl ops::Mul for Bytes {
    type Output = Self;

    fn mul(self, rhs: Bytes) -> Bytes {
        let mut res = Bytes::new(vec![0]);
        let mut rhs_copy = rhs;       


        while !rhs_copy.is_equal(&Bytes::new(vec![0])) {
            res = res + Bytes::new(self.bytes.clone());
            rhs_copy = rhs_copy - Bytes::new(vec![1]);
        }

        return res
    }
}

impl ops::Sub for Bytes {
    type Output = Self;

    fn sub(self, rhs: Bytes) -> Bytes {
        let mut res_bytes = Vec::new();

        let mut brw_bit = 0b0;
        for i in 0..self.bytes.len() {
            let lhs_byte = self.bytes.get(i).unwrap_or(&0b0);
            let rhs_byte = rhs.bytes.get(i);

            let (val, brw) = match rhs_byte {
                Some(val) => Bytes::_binary_sub(lhs_byte, val, 
                                                brw_bit),
                None => Bytes::_binary_sub(lhs_byte, &0b0, 
                                                brw_bit)
            };
            res_bytes.push(val);
            brw_bit = brw;
        }

        return Bytes::new(res_bytes)
    }
}

impl ops::Add for Bytes {
    type Output = Self;

    fn add(self, rhs: Bytes) -> Bytes {
        let mut res_bytes = Vec::new();
        let longest: Bytes;
        let shortest: Bytes;

        if self.bytes.len() > rhs.bytes.len() {
            longest = self;
            shortest = rhs;
        } else {
            longest = rhs;
            shortest = self;
        }

        let mut car_bit = 0b0;
        for i in 0..longest.bytes.len() {
            let lhs_byte = longest.bytes.get(i);
            let rhs_byte = shortest.bytes.get(i);

            let (val, car) = match rhs_byte {
                Some(val) => Bytes::_binary_add(lhs_byte.unwrap(), val, 
                                                car_bit),
                None => Bytes::_binary_add(lhs_byte.unwrap(), &0b0, 
                                                car_bit)
            };
            res_bytes.push(val);
            car_bit = car;
        }

        if car_bit > 0b0 {
            res_bytes.push(car_bit);
        }

        return Bytes::new(res_bytes)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_add() {
        let (res, _) = Bytes::_binary_add(&3, &5, 0);
        assert_eq!(res, 8 as u8)
    }
}