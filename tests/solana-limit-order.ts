import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaLimitOrder } from "../target/types/solana_limit_order";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("solana-limit-order", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaLimitOrder as Program<SolanaLimitOrder>;
  const SOL_coin = "0x41848d32f281383f214c69b7b248dc7c2e0a7374";
  const ETH_coin = "0x2170ed0880ac9a755fd29b2688956bd959f933f8";
  const sendOrder = async (trader: any, sell_coin: string, buy_coin: string, limit_price: any, sell_amount: any ) => {
    const order = anchor.web3.Keypair.generate();
    await program.rpc.createOrder(sell_coin, buy_coin, limit_price, sell_amount, {
      accounts: {
        order: order.publicKey,
        trader,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [order],
    });
    return order
}

  it('can create a limit order', async () => {
    // Call the "create_order" instruction.
    const order = anchor.web3.Keypair.generate();
    console.log('-----------> 1');
    console.log(order.publicKey)
    console.log('-----------> 2');
    console.log(program.provider.wallet.publicKey)
    console.log('-----------> 3');
    console.log(anchor.web3.SystemProgram.programId)
    await program.rpc.createOrder(ETH_coin, SOL_coin, 43.586388, 3.7542, {
      accounts: {
        order: order.publicKey,
        trader: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [order],
    });

    // Fetch the account details of the created order.
    const orderAccount = await program.account.order.fetch(order.publicKey);

    // Ensure it has the right data.
    assert.equal(orderAccount.trader.toBase58(), program.provider.wallet.publicKey.toBase58());
    assert.equal(orderAccount.sell_coin, ETH_coin);
    assert.equal(orderAccount.buy_coin, SOL_coin);
    assert.equal(orderAccount.limit_price, 43.586388);
    assert.equal(orderAccount.sell_amount, 3.7542);
  });

  it('can delete a order', async () => {
    // Create a new order.
    const trader = program.provider.wallet.publicKey;
    const order = await sendOrder(trader, ETH_coin, SOL_coin, 43.5242, 10.3);

    // Delete the order.
    await program.rpc.deleteOrder({
        accounts: {
            order: order.publicKey,
            trader,
        },
    });

    // Ensure fetching the order account returns null.
    const orderAccount = await program.account.order.fetchNullable(order.publicKey);
    assert.ok(orderAccount === null);
});
});
