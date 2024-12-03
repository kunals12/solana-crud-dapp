import {Program} from '@coral-xyz/anchor'
import {PublicKey} from '@solana/web3.js'
import {Crudapp} from '../target/types/crudapp'
import { BankrunProvider, startAnchor } from 'anchor-bankrun'

const IDL = require("../target/idl/crudapp.json");
const crudAppAddress = new PublicKey("D9fofwkjqYzDUUHKudbQ2UVxRDji7gwFx8aJJx1bdkbp");

describe('crudapp', () => {
  let context;
  let provider: BankrunProvider;
  let crudAppProgram: Program<Crudapp>;

  beforeAll(async() => {
    context = await startAnchor("", [{name: "crudapp", programId: crudAppAddress}], []);
    provider = new BankrunProvider(context);
    crudAppProgram = new Program<Crudapp>(IDL, provider);
  })
  
  it("create entry", async() => {
    const title = "Hello";
    await crudAppProgram.methods.createJournalEntry(
      title,
      "Hello World"
    ).rpc();
    
    const [entryAddress] = PublicKey.findProgramAddressSync([Buffer.from(title), provider.wallet.publicKey.toBuffer()], crudAppAddress);
    // console.log({entryAddress});
    
    const entry = await crudAppProgram.account.journalEntryState.fetch(entryAddress)
    // console.log(entry);
    expect(entry.title).toEqual(title);
  });

  it("update entry", async() => {
    const title = "Hello";
    await crudAppProgram.methods.updateJournalEntry(title, "Hello world 2").rpc();

    const [entryAddress] = PublicKey.findProgramAddressSync([Buffer.from(title), provider.wallet.publicKey.toBuffer()], crudAppAddress);
    const entry = await crudAppProgram.account.journalEntryState.fetch(entryAddress);
    console.log({entry});
    expect(entry.message).toEqual("Hello world 2");
    expect(entry.title).toEqual("Hello");
  })

  it("delete entry", async() => {
    const title = "Hello";
    await crudAppProgram.methods.deleteJournalEntry(title).rpc();
    // const [entryAddress] = PublicKey.findProgramAddressSync([Buffer.from(title), provider.wallet.publicKey.toBuffer()], crudAppAddress);

  })
})
