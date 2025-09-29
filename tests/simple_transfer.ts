import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleTransfer } from "../target/types/simple_transfer";
import assert from "assert";

describe("simple_transfer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.simple_transfer as Program<SimpleTransfer>;
  const wallet = provider.wallet;

  describe("create_account ix", () => {
    it("should create a new account", async () => {
      const _title = "My first account";
      await program.methods.createAccount(_title).rpc();

      const [saving_account_pda, bump_seed] =
        anchor.web3.PublicKey.findProgramAddressSync(
          [wallet.publicKey.toBuffer(), Buffer.from(_title)],
          program.programId
        );

      const savingAccount = await program.account.savingAccount.fetch(
        saving_account_pda
      );
      assert(savingAccount.title === _title);
      assert(savingAccount.bumpSeed === bump_seed);
    });
  });

  describe("deposit ix", () => {
    it("should send lamports to saving_account", async () => {
      const _title = "My first account";
      const _amount = new anchor.BN(1 * 1_000_000_000);
      const [saving_account_pda] = anchor.web3.PublicKey.findProgramAddressSync(
        [wallet.publicKey.toBuffer(), Buffer.from(_title)],
        program.programId
      );

      const initBalance = await provider.connection.getBalance(
        saving_account_pda
      );

      await program.methods.deposit(_title, _amount).rpc();

      const finalBalance = await provider.connection.getBalance(
        saving_account_pda
      );
      assert(finalBalance - initBalance === _amount.toNumber());
    });
  });
});
