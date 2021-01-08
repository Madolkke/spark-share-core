const KBYTE_L: u64 = 0xFFFF_FFFF_FFFF_FC00;
const KBYTE_R: u64 = 0x0000_0000_0000_03FF;
const MBYTE_L: u64 = 0xFFFF_FFFF_FFF0_0000;
const MBYTE_R: u64 = 0x0000_0000_000F_FFFF;
const GBYTE_L: u64 = 0xFFFF_FFFF_C000_0000;
const GBYTE_R: u64 = 0x0000_0000_3FFF_FFFF;
const TBYTE_L: u64 = 0xFFFF_FF00_0000_0000;
const TBYTE_R: u64 = 0x0000_00FF_FFFF_FFFF;
const PBYTE_L: u64 = 0xFFFC_0000_0000_0000;
const PBYTE_R: u64 = 0x0003_FFFF_FFFF_FFFF;
const EBYTE_L: u64 = 0xF000_0000_0000_0000;
const EBYTE_R: u64 = 0x0FFF_FFFF_FFFF_FFFF;
#[derive(Debug)]
pub enum SizeScale{
    Byte, KByte, MByte, GByte, TByte, PByte, EByte
}
pub struct Size(u64);

impl Size{

    pub fn new(byte_size: u64) -> Size{
        Size(byte_size)
    }

    pub fn to_bytes(self) -> [u8; 8]{
        self.0.to_be_bytes()
    }

    pub fn each_1024(&self) -> [u16; 7]{
        let mut result: [u16; 7] = [0; 7];
        let mut mid = self.0.clone();
        let mut counter = 0;
        while counter < 7{
            result[counter] = (mid & 0x3ff) as u16;
            mid = mid >> 10;
            counter += 1;
        }
        return result;
    }

    pub fn float_by_scale(&self, scale: SizeScale) -> f64{
        let k = self.0;
        match scale{
            SizeScale::Byte => self.0 as f64,
            SizeScale::KByte => ((k&KBYTE_L) >> 10)as f64 + (k&KBYTE_R)as f64 / KBYTE_R as f64,
            SizeScale::MByte => ((k&MBYTE_L) >> 20)as f64 + (k&MBYTE_R)as f64 / MBYTE_R as f64,
            SizeScale::GByte => ((k&GBYTE_L) >> 30)as f64 + (k&GBYTE_R)as f64 / GBYTE_R as f64,
            SizeScale::TByte => ((k&TBYTE_L) >> 40)as f64 + (k&TBYTE_R)as f64 / TBYTE_R as f64,
            SizeScale::PByte => ((k&PBYTE_L) >> 50)as f64 + (k&PBYTE_R)as f64 / PBYTE_R as f64,
            SizeScale::EByte => ((k&EBYTE_L) >> 60)as f64 + (k&EBYTE_R)as f64 / EBYTE_R as f64,
        }
    }

    pub fn auto_float_by_scale(&self) -> (SizeScale, f64){
        use SizeScale::*;
        let a = self.each_1024();
        let mut counter = 6;
        loop{
            if a[counter] != 0 {
                return match counter{
                    6 => (EByte, self.float_by_scale(EByte)),
                    5 => (PByte, self.float_by_scale(PByte)),
                    4 => (TByte, self.float_by_scale(TByte)),
                    3 => (GByte, self.float_by_scale(GByte)),
                    2 => (MByte, self.float_by_scale(MByte)),
                    1 => (KByte, self.float_by_scale(KByte)),
                    0 => (Byte, self.float_by_scale(Byte)),
                    _ => panic!()
                }
            }
            if counter != 0 { counter -= 1;}
            else {break;}
        }
        return (SizeScale::Byte, 0 as f64)
    }

    pub fn to_detailed_string(&self) -> String{
        let mut result = String::new();
        let a = self.each_1024();
        if a[6] != 0 { result += &format!("{}EB+ ", a[6]); }
        if a[5] != 0 { result += &format!("{}PB+ ", a[5]); }
        if a[4] != 0 { result += &format!("{}TB+ ", a[4]); }
        if a[3] != 0 { result += &format!("{}GB+ ", a[3]); }
        if a[2] != 0 { result += &format!("{}MB+ ", a[2]); }
        if a[1] != 0 { result += &format!("{}KB+ ", a[1]); }
        if a[0] != 0 { result += &format!("{}B", a[0]); }
        if &result == "" { result = "0B".to_string(); }
        return result;
    }
}

impl std::ops::Add for Size{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Size(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Size{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Size{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Size(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Size{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl std::fmt::Display for Size{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}