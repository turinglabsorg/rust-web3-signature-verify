use web3::signing::{keccak256, recover};

#[tokio::main]
pub async fn main() -> web3::Result<()> {

    let message = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string();
    let sig = "382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b".to_string();
    let message = eth_message(message);
    let signature = hex::decode(sig).unwrap();
    let pubkey = recover(&message, &signature[..64], 0);

    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    println!("Verified pubkey is: {}", pubkey);
    
    Ok(())
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}
