import * as anchor from '@coral-xyz/anchor';
import { describe, it } from 'mocha';
import { expect } from 'chai';
import { Connection, PublicKey, Keypair, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';
import fs from "fs";
import BN from "bn.js";

const idl = JSON.parse(
  fs.readFileSync("./idl/pinocchio_price_account.json", "utf-8")
);

console.log("IDL metadata:", idl.metadata);
console.log("IDL address:", idl.metadata?.address);
console.log("Full IDL structure:", JSON.stringify(idl, null, 2));

if (!idl.metadata?.address) {
    throw new Error("No address found in IDL metadata");
}

describe('Create Price Account', function() {
    this.timeout(10000);
    
    let connection: Connection;
    let program: Program;
    let provider: AnchorProvider;
    
    const priceAccountId = new BN(834);
    let priceAccountPda: PublicKey;

    before(async function () {
        connection = new Connection('https://api.devnet.solana.com', 'confirmed');

        let payer: Keypair;
        try {
            const secretKey = JSON.parse(fs.readFileSync('./wallet.json', 'utf8'));
            payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));
            console.log("Successfully loaded wallet from wallet.json");
            console.log("Wallet Public Key:", payer.publicKey.toString());
        } catch (error) {
            console.error("Error loading wallet from wallet.json:", error);
            console.log("Generating a new temporary Keypair instead.");
            payer = Keypair.generate();
        }

        const wallet = new Wallet(payer);
        provider = new AnchorProvider(connection, wallet, { commitment: 'confirmed' });
        anchor.setProvider(provider);

        console.log("About to create Program with:");
        console.log("- IDL type:", typeof idl);
        console.log("- IDL keys:", Object.keys(idl));
        console.log("- Provider type:", typeof provider);

        try {
            program = new Program(
                idl as anchor.Idl,
                idl.metadata.address, 
                provider
            );

            (program as any)._programId = new PublicKey(idl.metadata.address);

            console.log("Program created successfully");
        } catch (error) {
            console.error("Error creating program:", error);
            console.log("IDL metadata structure:", idl.metadata);
            throw error;
        }

        console.log("payer.publicKey:", provider.wallet.publicKey.toString());
        console.log("programId:", program.programId.toString());
        console.log("program methods: ", program.methods);
        console.log("Accounts : ", program.account);
        
        [priceAccountPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("price_feed_account"), priceAccountId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );
        console.log("Price Account PDA:", priceAccountPda.toString());
    });

    it("Should create Price Account", async () => {
        const [priceAccountPda, bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("price_feed_account"), priceAccountId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );

        console.log("Creating Price Account at PDA:", priceAccountPda.toString());
        
        const instructionDiscriminant = Buffer.from([0]); // Per the IDL
        const priceAccountIdBuffer = priceAccountId.toArrayLike(Buffer, "le", 8);
        const instructionData = Buffer.concat([instructionDiscriminant, priceAccountIdBuffer]);

        const ix = new TransactionInstruction({
            programId: program.programId,
            keys: [
                { pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: priceAccountPda, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            data: instructionData,
        });

        const tx = new Transaction().add(ix);

        const sig = await provider.sendAndConfirm(tx, []);
        console.log("Transaction Signature:", sig);

        const accountInfo = await connection.getAccountInfo(priceAccountPda);
        if (!accountInfo) {
            throw new Error("Account not found");
        }
        console.log(`Account Info: ${accountInfo}`)

        const accountData = accountInfo.data;

        const price = accountData.readDoubleLE(0); 
        const timestamp = accountData.readBigInt64LE(8); 
        const bumpFromAccount = accountData.readUInt8(16);

        console.log("Account data:", {
            price,
            lastUpdatedTimestamp: timestamp.toString(),
            priceAccountBump: bumpFromAccount,
        });
    });

    it("Set Price", async () => {
        const [priceAccountPda, bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("price_feed_account"), priceAccountId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );

        const instructionDiscriminant = Buffer.from([1]); // Per the IDL
        const priceAccountIdBuffer = priceAccountId.toArrayLike(Buffer, "le", 8);
        const priceToSet = 100;

        const priceBuffer = Buffer.allocUnsafe(8);
        priceBuffer.writeDoubleLE(priceToSet, 0);

        const instructionData = Buffer.concat([instructionDiscriminant, priceAccountIdBuffer, priceBuffer]);

        const ix = new TransactionInstruction({
            programId: program.programId,
            keys: [
                { pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: priceAccountPda, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            data: instructionData,
        });

        const tx = new Transaction().add(ix);

        const sig = await provider.sendAndConfirm(tx, []);
        console.log("Transaction Signature:", sig);

        const accountInfo = await connection.getAccountInfo(priceAccountPda);
        if (!accountInfo) {
            throw new Error("Account not found");
        }
        console.log(`Account Info: ${accountInfo}`)

         const accountData = accountInfo.data;

        const price = accountData.readDoubleLE(0); 
        const timestamp = accountData.readBigInt64LE(8); 
        const bumpFromAccount = accountData.readUInt8(16);

        console.log("Account data:", {
            price,
            lastUpdatedTimestamp: timestamp.toString(),
            priceAccountBump: bumpFromAccount,
        });
    });

    it("Modify Price", async () => {
        const [priceAccountPda, bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("price_feed_account"), priceAccountId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );

        const instructionDiscriminant = Buffer.from([2]);
        const priceAccountIdBuffer = priceAccountId.toArrayLike(Buffer, "le", 8);
        const modifiedPrice = 140;

        const priceBuffer = Buffer.allocUnsafe(8);
        priceBuffer.writeDoubleLE(modifiedPrice, 0);

        const instructionData = Buffer.concat([instructionDiscriminant, priceAccountIdBuffer, priceBuffer]);

        const ix = new TransactionInstruction({
            programId: program.programId,
            keys: [
                { pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true },
                { pubkey: priceAccountPda, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            data: instructionData,
        });

        const tx = new Transaction().add(ix);

        const sig = await provider.sendAndConfirm(tx, []);
        console.log("Transaction Signature:", sig);

        const accountInfo = await connection.getAccountInfo(priceAccountPda);
        if (!accountInfo) {
            throw new Error("Account not found");
        }
        console.log(`Account Info: ${accountInfo}`)

         const accountData = accountInfo.data;

        const price = accountData.readDoubleLE(0); 
        const timestamp = accountData.readBigInt64LE(8); 
        const bumpFromAccount = accountData.readUInt8(16);

        console.log("Account data:", {
            price,
            lastUpdatedTimestamp: timestamp.toString(),
            priceAccountBump: bumpFromAccount,
        });
    })
});