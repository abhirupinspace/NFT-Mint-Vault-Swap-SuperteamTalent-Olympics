use anchor_lang::prelude::*;
use solana_program_test::*;
use nft_minter::*;

#[tokio::test]
async fn test
_mint_nft() {
    // Setup
    let program = ProgramTest::new("nft_minter", id(), processor!(nft_minter::nft_minter::Program::new));
    let mut client = program.start().await;
    // Test minting function
    let mut transaction = Transaction::new_with_payer(
    &[Instruction::new_with_bincode(
        id(),
        &MintNFT {
            uri: "test_uri".to_string(),
            ..MintNFT::default()
        },
        vec![],
    )],
    Some(&client.payer.pubkey()),
);
transaction.sign(&[&client.payer]);
let result = client.process_transaction(transaction).await;
assert!(result.is_ok());
}