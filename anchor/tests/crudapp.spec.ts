import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
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
})
