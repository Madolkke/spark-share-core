const GB_CONVERT_u64: u64 = 1_073_741_824;
const GB_CONVERT_f64: f64 = 1_073_741_824.0;
pub struct Size(u64);

impl Size{

    pub fn new(byte_size: u64) -> Size{
        Size(byte_size)
    }

    pub fn to_bytes(self) -> [u8; 8]{
        self.0.to_be_bytes()
    }

    pub fn gb_f64(&self) -> f64{
        (self.0 / GB_CONVERT_u64) as f64 + (self.0 % GB_CONVERT_u64) as f64 / GB_CONVERT_f64
    }

    pub fn each_1024(&self) -> [u16; 7]{
        let mut result: [u16; 7] = [0; 7];
        let mut mid = self.0.clone();
        let mut counter = 0;
        while counter < 7{
            result[counter] = (mid & 0x3ff) as u16;
            mid = mid >> 10;
            println!("{}", mid);
            counter += 1;
        }
        return result;
    }

    
}