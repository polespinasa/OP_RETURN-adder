use std::env;
use std::process;
use hex::decode as hex_decode;
use bitcoin::blockdata::{opcodes, script::Builder};
use bitcoin::TxOut;
use bitcoin::consensus::encode::{deserialize, serialize_hex};
use bitcoin::Transaction;
use bdk::electrum_client::{Client, ElectrumApi};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    assert!(args.len() == 3, "Error: 2 args must be provided but {} where given.", args.len() - 1);

    match hex::decode(&args[1]) {
        Ok(data) => {
            match deserialize::<Transaction>(&data) {
                Ok(mut tx) => {
                    // If we get to this point we know that the first argument is a bitcoin transaction in the correct format

                    // Second argument must be the OP_RETURN data
                    let data = args[2].as_bytes();

                    // Create the OP_RETURN script
                    let script = Builder::new()
                        .push_opcode(opcodes::all::OP_RETURN)
                        .push_slice(data)
                        .into_script();

                    // Create the OP_RETURN output with value 0
                    let tx_out = TxOut {
                        value: 0,
                        script_pubkey: script,
                    };

                    // Add the OP_RETURN output to the transaction and serialize it
                    tx.output.push(tx_out);
                    let tx_hex = serialize_hex(&tx);
                    let tx_bytes = hex_decode(tx_hex).unwrap();

                    // MAINNET
                    //let ELECTRUM_ENDPOINT: &str = "ssl://electrum.blockstream.info:50001";
                    
                    // TESTNET
                    let ELECTRUM_ENDPOINT: &str = "ssl://electrum.blockstream.info:60002";

                    
                    let client = Client::new(ELECTRUM_ENDPOINT).unwrap();
            
                    // broadcast the transaction
                    let txid = client.transaction_broadcast_raw(&tx_bytes);

                    // There's a format error with the txid variable
                    //println!("Transaction broadcasted with TXID: {}", txid);
                },
                Err(_) => {
                    eprintln!("Error: The first argument is not a valid raw bitcoin transaction.");
                    process::exit(1);
                }
            }
        },
        Err(_) => {
            eprintln!("Error: The first argument is not a valid raw bitcoin transaction.");
            process::exit(1);
        }
    }
}