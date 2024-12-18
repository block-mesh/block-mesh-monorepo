import { createCloseAccountInstruction } from '@solana/spl-token'
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction
} from '@solana/web3.js'

const CYCOIN: PublicKey = new PublicKey('J6GJFgdtKeXFCCNXbvWw5v2yf81FjMy8i7mi9To9WXJL')

export type CloseAccountInstructionInput = {
  account: PublicKey,
  destination: PublicKey,
  authority: PublicKey,
}

function close_account_instruction(input: CloseAccountInstructionInput): [TransactionInstruction, number] {
  const inst = createCloseAccountInstruction(input.account, input.destination, input.authority)
  return [inst, 200]
}


export async function build_transaction(connection: Connection, owner: PublicKey, input: CloseAccountInstructionInput[]): Promise<Transaction[]> {
  const block = await connection.getLatestBlockhash()
  const instructions: TransactionInstruction[][] = []
  let current_instructions: TransactionInstruction[] = []
  const transactions: Transaction[] = []
  let total = 0
  while (input.length > 0) {
    const i = input.pop()
    if (!i) {
      continue
    }
    const [inst, size] = close_account_instruction(i)
    if (size + total < 1200) {
      current_instructions.push(inst)
      total += size
    } else {
      instructions.push(current_instructions)
      current_instructions = []
      current_instructions.push(inst)
      total = size
    }
  }
  instructions.push(current_instructions)
  for (const i_array of instructions) {
    const txn = new Transaction()
    txn.lastValidBlockHeight = block.lastValidBlockHeight
    txn.feePayer = owner
    txn.recentBlockhash = block.blockhash
    for (const i of i_array) {
      txn.add(i)
    }
    transactions.push(txn)
  }

  const transferTransaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: owner,
      toPubkey: CYCOIN,
      lamports: input.length * 0.002 * LAMPORTS_PER_SOL * 0.1
    })
  )
  transferTransaction.lastValidBlockHeight = block.lastValidBlockHeight
  transferTransaction.feePayer = owner
  transferTransaction.recentBlockhash = block.blockhash
  transactions.push(transferTransaction)
  return transactions
}