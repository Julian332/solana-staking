import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {TokenStake} from "../target/types/token_stake";
import {
    createMint, getAccount,
    getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintToChecked,
    TOKEN_PROGRAM_ID,
    transfer,
    transferChecked
} from "@solana/spl-token";
import {Keypair, PublicKey} from "@solana/web3.js";
import {SYSTEM_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/native/system";

describe("token-stake", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const connection = anchor.getProvider().connection;
    const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode("pool_state"))
    const DEPOSITOR_STATE = Buffer.from(anchor.utils.bytes.utf8.encode("depositor_state"))
    const program = anchor.workspace.TokenStake as Program<TokenStake>;
    const user = anchor.getProvider().publicKey;
    console.log("user:" + user.toString());
    const token = Keypair.generate();
    const payer = Keypair.generate();
    console.log("token:" + token.publicKey.toString());

    let programId = program.programId;
    const pool_state = PublicKey.findProgramAddressSync([
            POOL_SEED,
            token.publicKey.toBuffer(),
        ],
        programId)[0];
    const user_state = PublicKey.findProgramAddressSync([
            DEPOSITOR_STATE,
            token.publicKey.toBuffer(),
            user.toBuffer()
        ],
        programId)[0];
    console.log("pool_state:" + pool_state.toString());

    let pool_ata_addr = getAssociatedTokenAddressSync(token.publicKey, pool_state, true);
    console.log("pool_ata_addr:" + pool_ata_addr.toString());
    const decimal = 6;


    it("Full lifetime!", async () => {

        // Add your test here.

        try {
            await connection.requestAirdrop(payer.publicKey, 1000000000000);
            await delay(1000)
            const mint = await createMint(connection, payer, payer.publicKey, null, decimal, token);
            const tx = await program.methods.initialize().accounts({
                creator: user,
                poolState: pool_state,
                stakingTokenAta: pool_ata_addr,
                stakingToken: token.publicKey,
                systemProgram: SYSTEM_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).rpc();

            const payer_ata = await getOrCreateAssociatedTokenAccount(connection, payer, mint, payer.publicKey);
            let user_ata = await getOrCreateAssociatedTokenAccount(connection, payer, mint, user);
            await mintToChecked(connection, payer, mint, payer_ata.address, payer, 10000000000, decimal);
            await mintToChecked(connection, payer, mint, user_ata.address, payer, 1000000, decimal);

            await transferChecked(connection, payer, payer_ata.address, mint, pool_ata_addr, payer, 1000000, decimal);


            console.log("++++++++++++++");
            await program.methods.deposit(new BN(1000000)).accounts({
                depositor: user,

                poolState: pool_state,

                depositorState: user_state,

                depositorTokenAta: user_ata.address,

                poolTokenAta: pool_ata_addr,

                stakingToken: token.publicKey,
                systemProgram: SYSTEM_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).rpc();
            let user_state_data = await program.account.depositorState.fetch(user_state);
            let pool_state_data = await program.account.stakingState.fetch(pool_state);
            console.log("user lp"+user_state_data.depositorLp.toString());
            console.log("pool lp"+pool_state_data.totalLp.toString());
            user_ata = await getAccount(connection, user_ata.address,);
            let pool_ata = await getAccount(connection, pool_ata_addr,);
            console.log("user token balance"+user_ata.amount);
            console.log("pool token balance"+pool_ata.amount);


            await transferChecked(connection, payer, payer_ata.address, mint, pool_ata_addr, payer, 1000000, decimal);


            await program.methods.withdraw(user_state_data.depositorLp).accounts({
                depositor: user,

                poolState: pool_state,

                depositorState: user_state,

                depositorTokenAta: user_ata.address,

                poolTokenAta: pool_ata_addr,

                stakingToken: token.publicKey,
                systemProgram: SYSTEM_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).rpc();
            user_state_data = await program.account.depositorState.fetch(user_state);
            pool_state_data = await program.account.stakingState.fetch(pool_state);
            console.log("user lp"+user_state_data.depositorLp.toString());
            console.log("pool lp"+pool_state_data.totalLp.toString());
            user_ata = await getAccount(connection, user_ata.address,);
            pool_ata = await getAccount(connection, pool_ata_addr,);
            console.log("user token balance"+user_ata.amount);
            console.log("pool token balance"+pool_ata.amount);

        } catch (e) {
            console.log(e);
            throw e;
        }
        // console.log("Your transaction signature", tx);
    });
});

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}