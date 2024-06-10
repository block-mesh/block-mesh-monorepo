use std::fmt::Display;
#[derive(Debug)]
pub struct Metadata {
    pub city: String,
    pub country: String,
    pub ip: String,
    pub asn: String,
    pub colo: String,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "City: {}\nCountry: {}\nIp: {}\nAsn: {}\nColo: {}",
            self.city, self.country, self.ip, self.asn, self.colo
        )
    }
}
