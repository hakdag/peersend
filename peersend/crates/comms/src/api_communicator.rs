use core::{api::ApiAccess, token::TokenStorageAccessable};

pub struct APICommunicator<TTokenAccess>
    where TTokenAccess: TokenStorageAccessable {
    token_access: TTokenAccess
}

impl<TTokenAccess> APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    pub fn new(token_access: TTokenAccess) -> Self {
        Self { token_access }
    }
}

impl<TTokenAccess> ApiAccess for APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, std::io::Error> {
        todo!()
    }
    
    fn set_target_ipaddress(&self, ip_address: &String) -> Result<(), std::io::Error> {
        todo!()
    }
}
