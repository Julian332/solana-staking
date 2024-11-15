import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {TokenStake} from "../target/types/token_stake";
import {
    createMint,
    getAssociatedTokenAddressSync,
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
    const program = anchor.workspace.TokenStake as Program<TokenStake>;
    const user = anchor.getProvider().publicKey;
    console.log("user:"+user.toString());
    const token = Keypair.generate();
    const payer = Keypair.generate();
    console.log("token:"+token.publicKey.toString());

    let programId = program.programId;
    const pool_state = PublicKey.findProgramAddressSync([
            POOL_SEED,
            token.publicKey.toBuffer(),

        ],
        programId)[0];
    console.log("pool_state:"+pool_state.toString());

    const pool_ata = getAssociatedTokenAddressSync(token.publicKey, pool_state, true);
    console.log("pool_ata:"+pool_ata.toString());






    it("Is initialized!", async () => {

        // Add your test here.

        try {
            await connection.requestAirdrop(payer.publicKey,10000000000);
            await delay(1000)
            console.log(await connection.getBalance(payer.publicKey));
            const mint =await createMint(connection,payer,user,null,6,token);
            const tx = await program.methods.initialize().accounts({
                creator: user,
                poolState: pool_state.toBase58(),
                staking_tokenAta: pool_ata,
                stakingToken: token.publicKey,
                systemProgram: SYSTEM_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).rpc();
        } catch (e) {
            console.log(e);
        }
        // console.log("Your transaction signature", tx);
    });
});
function delay(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}