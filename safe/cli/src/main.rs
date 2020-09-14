use serum_common::Cluster;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "srm-safe", about = "A cli to interact with the Serum Safe")]
struct Opt {
    /// Program id on Solana.
    #[structopt(long)]
    program_id: Pubkey,

    /// Cluster identifier to communicate with.
    #[structopt(long)]
    cluster: Cluster,

    /// Path to the payer's keypair file.
    #[structopt(long)]
    payer_filepath: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world! {:?}", opt);

    let client = RpcClient::new(opt.cluster.url().to_string());

    let deposit_instruction = serum_safe::instruction::deposit(opt.program_id, 1);
    let instructions = vec![deposit_instruction];

    // todo: don't unwrap
    let (recent_hash, _fee_calc) = client.get_recent_blockhash().unwrap();

    // todo: don't expect here.
    let payer = solana_sdk::signature::read_keypair_file(&opt.payer_filepath)
        .expect("keypair must be provided");

    let txn = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer],
        recent_hash,
    );

    let result = client
        .send_and_confirm_transaction_with_spinner_and_config(
            &txn,
            CommitmentConfig::single(),
            RpcSendTransactionConfig {
                skip_preflight: true,
                preflight_commitment: None,
            },
        )
        .unwrap();
    println!("TX SIGNATURE {:?}", result);
}

// 29y9cwVcyLEJnLcjSduHUhWRkRYP67zwqgWeExH6gUzi
