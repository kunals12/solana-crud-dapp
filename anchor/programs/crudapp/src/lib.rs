#![allow(clippy::result_large_err)] // Allow large error types for simplicity

use anchor_lang::prelude::*; // Import the Anchor framework prelude

// Declare the program ID for the Solana program
declare_id!("D9fofwkjqYzDUUHKudbQ2UVxRDji7gwFx8aJJx1bdkbp");

#[program] // Define the main entry points for the program
pub mod crudapp {
    use super::*;

    // Create a new journal entry
    pub fn create_journal_entry(
        ctx: Context<CreateJournalEntry>, // Context contains accounts and instructions for this function
        title: String,                    // Title of the journal entry
        message: String,                  // Message content of the journal entry
    ) -> Result<()> {
        // Get a mutable reference to the journal entry account
        let journal_entry = &mut ctx.accounts.journal_entry;
        // Assign values to the account fields
        journal_entry.owner = *ctx.accounts.owner.key; // Set the owner to the signer
        journal_entry.title = title; // Set the title
        journal_entry.message = message; // Set the message
        Ok(()) // Return success
    }

    // Update an existing journal entry
    pub fn update_journal_entry(
        ctx: Context<UpdateJournalEntry>, // Context contains accounts and instructions
        _title: String,                   // Title of the journal entry (used for PDA derivation)
        message: String,                  // New message content to update
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message; // Update the message
        Ok(()) // Return success
    }

    // Delete an existing journal entry
    pub fn delete_journal_entry(
        _ctx: Context<DeleteJournalEntry>, // Context contains accounts and instructions
        _title: String,                    // Title of the journal entry (used for PDA derivation)
    ) -> Result<()> {
        Ok(()) // Close the account; no additional logic needed here
    }
}

// Context for creating a journal entry
#[derive(Accounts)]
#[instruction(title: String)] // Instruction to pass the title as an argument
pub struct CreateJournalEntry<'info> {
    #[account(mut)] // The account that will pay for the new account
    pub owner: Signer<'info>, // The signer of the transaction (payer and owner of the account)

    #[account(
        init, // Initialize a new account
        seeds = [title.as_bytes(), owner.key().as_ref()], // Derive PDA using title and owner's public key
        space = 8 + JournalEntryState::INIT_SPACE, // Allocate space for the account
        payer = owner, // Specify the payer for account creation
        bump // Auto-generate a bump seed for the PDA
    )]
    pub journal_entry: Account<'info, JournalEntryState>, // The journal entry account

    pub system_program: Program<'info, System>, // System program for account creation
}

// Context for updating a journal entry
#[derive(Accounts)]
#[instruction(title: String)] // Instruction to pass the title as an argument
pub struct UpdateJournalEntry<'info> {
    #[account(mut)] // The signer who owns the account
    pub owner: Signer<'info>,

    #[account(
        mut, // Allow modification of the account
        seeds = [title.as_bytes(), owner.key().as_ref()], // Derive PDA using title and owner's public key
        realloc = 8 + JournalEntryState::INIT_SPACE, // Resize account to ensure sufficient space
        realloc::payer = owner, // Specify the payer for resizing
        realloc::zero = true, // Zero out the excess bytes after resizing
        bump // Auto-generate a bump seed for the PDA
    )]
    pub journal_entry: Account<'info, JournalEntryState>, // The journal entry account

    pub system_program: Program<'info, System>, // System program for account resizing
}

// Context for deleting a journal entry
#[derive(Accounts)]
#[instruction(title: String)] // Instruction to pass the title as an argument
pub struct DeleteJournalEntry<'info> {
    #[account(mut)] // The signer who owns the account
    pub owner: Signer<'info>,

    #[account(
        mut, // Allow modification of the account
        seeds = [title.as_bytes(), owner.key().as_ref()], // Derive PDA using title and owner's public key
        bump, // Auto-generate a bump seed for the PDA
        close = owner, // Transfer remaining SOL to the owner upon account closure
    )]
    pub journal_entry: Account<'info, JournalEntryState>, // The journal entry account

    pub system_program: Program<'info, System>, // System program for account deletion
}

// Data structure for a journal entry account
#[account]
#[derive(InitSpace)] // Automatically calculate the space needed for the account
pub struct JournalEntryState {
    pub owner: Pubkey, // Owner of the journal entry
    #[max_len(50)] // Maximum length of the title is 50 characters
    pub title: String, // Title of the journal entry
    #[max_len(500)] // Maximum length of the message is 500 characters
    pub message: String, // Message content of the journal entry
}
