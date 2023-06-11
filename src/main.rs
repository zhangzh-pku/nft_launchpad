use web3::contract::Contract;
use web3::types::Address;
use web3::futures::Future;

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);


    let contract_address: Address = "0x...".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("contract_abi.json"),
    ).unwrap();

    let result = contract.query("myMethod", (), None, web3::contract::Options::default(), None);
    let some_data: String = result.wait().unwrap();
    println!("Data: {:?}", some_data);
}
