#[derive(Debug)]
pub struct Parameters {
    pub bits: usize,
    pub t: usize,
    pub rhu: usize,
}

pub fn calculate_parameters(lambda: usize, t: usize) -> Parameters {
    let bits = match lambda {
        80 => 512,           // if lambda=80, N is 3072 bits, so p and q are 512 bits
        92 => 768,           // if lambda=92, N is 3072 bits, so p and q are 768 bits
        112 => 1024,         // if lambda=112, N is 3072 bits, so p and q are 1024 bits
        // 128 => 1536,       // if lambda=128, for tau=1 and l=11, N is 3072 bits, so p and q are 1536 bits
        // 128 => 6551,       // if lambda=128, for tau=1 and l=50, N is 13102 bits, so p and q are 6551 bits
        // 128 => 13101,      // if lambda=128, for tau=1 and l=100, N is 26202 bits, so p and q are 13101 bits
        // 128 => 1536,       // if lambda=128, for tau=8 and l=11, N is 3072 bits, so p and q are 1536 bits        
        // 128 => 6901,       // if lambda=128, for tau=8 and l=50, N is 13802 bits, so p and q are 6901 bits
        //128 => 13801,      // if lambda=128, for tau=8 and l=100, N is 27602 bits, so p and q are 13801 bits
        // 128 => 1536,       // if lambda=128, for tau=16 and l=10, N is 3072 bits, so p and q are 1536 bits               
        // 128 => 7301,       // if lambda=128, for tau=16 and l=50, N is 14602 bits, so p and q are 7301 bits
        // 128 => 14601,       // if lambda=128, for tau=16 and l=100, N is 29202 bits, so p and q are 14601 bits        
        // 128 => 1536,       // if lambda=128, for tau=32 and l=9, N is 3072 bits, so p and q are 1536 bits
        128 => 8101,       // if lambda=128, for tau=32 and l=50, N is 16202 bits, so p and q are 8101 bits
        // 128 => 14601,       // if lambda=128, for tau=16 and l=100, N is 29202 bits, so p and q are 14601 bits
        _ => panic!("Unsupported lambda value"),
    };
    let rhu = lambda+2;

    Parameters { bits, t, rhu }
}