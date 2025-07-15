use std::fmt::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: Vec<u8>) -> Swap;
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Swap {
        if v.len() < 8 {
            return Err(Error);
        }
       let qty_1 = v[0..3]; 
       let qty_2 = v[4..7]; 
       
       return OK(Swap {
           qty_1,
           qty_2
        })
    }
}


struct  Swap {
    qty_1: u32,
    qty_2: u32
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());

        v
    }

    
}