use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, String /* &'static str*/>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, String /*&'static str*/> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(error) => {
                //println!("Address::from_str error: {}", error);
                //See: https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
                let err = format!("Invalid ethereum address string due to following error: {}", error);
                Err(err)
            }
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, String /*&'static str*/> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address = address.convert_address().unwrap();
    //...do some stuff
    converted_address
}

fn print_name(name: &mut String) {
    name.push('!');
    println!("name = {}", name);
}

fn get_name() -> String {
    let name = format!("Here we are {}", true);
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_poly() {
        let mut name: String = String::from("Something else");
        print_name(&mut name);
        print_name(&mut name);
        let mut test = get_name();
        println!("get_name() returned {}", test);
        print_name(&mut test);

        let master_addr = Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap();

        let addr1 = get_ethereum_data(master_addr);
        assert_eq!(master_addr, addr1);

        let addr2 = get_ethereum_data("0x388C818CA8B9251b393131C08a736A67ccB19297");
        assert_eq!(master_addr, addr2);

        //This will fail: "Invalid Ethereum Address String"
        //let addr3 = get_ethereum_data("0x388C818CA8B9251b393131C08a736A67ccB1929P");
        //assert_eq!(master_addr, addr3);
    }
}