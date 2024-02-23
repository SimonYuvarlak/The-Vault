# The Vault

This is a `Cosmwasm` smart contract written in `Rust`.
This contract has not been tested in real world, so it should be used carefully after making the real world tests.

## Contract Functionality

The goal of this contract is to create a shared pool of tokens. Different addresses can claim their allowances from the pool if they qualify. So, with this sense, this contract can be seen as a safe to hold tokens and share with addresses.
Also, this contract is aimed to work as a salary payment method. The employees can see that their tokens are there and allocated for them. They can calim in the given time frame the given amount (this timing functionality has not been added yet). 
On thge other hand, the employers will not need to pay individuals or create systems. They can basically hold enough tokens in the pool and give allowance. In the future upgrades, a document will be created showing the token has been claimed. 
Finally, since other addresses can deposit here (it is restricted to allowed addresses with smart contract functionality but it is not possible to prevent sending tokens to the contract), this pool can work as a fully automated salary payment system.
Some projects can fill the pool while other addresses claim and get their tokens. In the future updates, it will be possible to get documents for depositing tokens.

- This contract validates certain addresses which can deposit tokens to the contract.
- This contract can give allowance to addresses which can retrieve tokens from pool based on the amount that is allocated for them.
- Only the owner of the contract can validate deposit addresses or give allowance to other addresses.
- How much token the contract holds in total and how much token has each address has deposited so far can be seen.

  ## Prereqs

  To run the program, you need to be sure that `Rust` and `Cargo` has been installed and updated. To work on the contract and deploy it, you will need to have the prereqs in the cosmwasm documentation.

  ### Build

  You can build the program with the `cargo build` command. This checks if the given rust code can be successfully compiled.


  ### Testing

  You can test the program with the `cargo test` command.

  ### Creationg Web Assembly Code

  Because of a compatibility issue between cosmwasm-std: 1.5.3 and cw-multi-test: 0.20.0, they will not work when they are together.
  So before creating the wasm file, delete the comment out the import line in the lib.rs `mod test` is the line that needs to be commented out.
  After the lib.rs file, also comment out the line in the `cargo.toml` -> `#cw-multi-test = "0.20.0"`
  Finally, rebuild the project with `cargo build`.

  To create a  `wasm` file, you can type `cargo wasm`.
  After creating the wasm file, you can check if the Rust code you have is a valid cosmwasm code, by typing `cosmwasm-check ./target/wasm32-unknown-unknwon/release/the_vault.wasm`.
  
