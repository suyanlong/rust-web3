extern crate cita_crypto;
extern crate protobuf;
extern crate rustc_hex;
extern crate tokio_core;
extern crate web3;

use cita_crypto::*;
use web3::futures::Future;
use web3::cita_types::{Transaction, UnverifiedTransaction};

//use std::str::FromStr;
use protobuf::core::Message;
use rustc_hex::FromHex;
use rustc_hex::ToHex;

const MAX_PARALLEL_REQUESTS: usize = 64;
const CONTRUCT_CODE: &str = "60606040523415600e57600080fd5b5b5b5b60948061001f6000396000f300\
                             60606040526000357c01000000000000000000000000000000000000000000\
                             00000000000000900463ffffffff1680635524107714603d575b600080fd5b\
                             3415604757600080fd5b605b6004808035906020019091905050605d565b00\
                             5b806000819055505b505600a165627a7a72305820c471b4376626da2540b2\
                             374e8b4110501051c426ff46814a6170ce9e219e49a80029";


fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let web3 = web3::Web3::new(
        web3::transports::Http::with_event_loop(
            "http://localhost:1337",
            &event_loop.handle(),
            MAX_PARALLEL_REQUESTS,
        ).unwrap(),
    );

    //study create sendtransaction param
    let cita = web3.cita();
    let height = cita.block_number().map(|height| {
        println!("height: {:?}", height);
        height
    });


    let number = event_loop.run(height).unwrap();
    let key_pair = KeyPair::gen_keypair();

    //create contract
    let number = number.low_u64();
    println!("number: {:?}", number);
    let tx = generate_tx(
        CONTRUCT_CODE.to_string(),
        "".to_string(),
        key_pair.privkey(),
        number,
        2500,
    ).write_to_bytes()
        .unwrap()
        .to_hex();

    println!("hex = {:?}", tx);
    let tx = cita.send_transaction(tx).map(|tx_response| {
        println!("tx_response: {:?}", tx_response);
        tx_response
    });
    event_loop.run(tx).unwrap();
}

fn generate_tx(code: String, address: String, pv: &PrivKey, curh: u64, quota: u64) -> UnverifiedTransaction {
    let data = code.from_hex().unwrap();
    let mut tx = Transaction::new();
    tx.set_data(data);
    tx.set_to(address);
    tx.set_nonce("0".to_string());
    tx.set_valid_until_block(curh + 100);
    tx.set_quota(quota);
    tx.sign(*pv).take_transaction_with_sig()
}
