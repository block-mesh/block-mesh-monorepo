import { createCloseAccountInstruction } from '@solana/spl-token'
import { Connection, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js'

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
  console.log('build_transaction block', block)
  console.log('build_transaction input', input)
  const instructions: TransactionInstruction[][] = []
  let current_instructions: TransactionInstruction[] = []
  const transactions: Transaction[] = []
  let total = 0
  while (input.length > 0) {
    const i = input.pop()
    console.log('build_transaction', i)
    if (!i) {
      console.log('build_transaction continue')
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
  console.log('build_transaction here')
  for (const i_array of instructions) {
    console.log('build_transaction i_array', i_array)
    const txn = new Transaction()
    txn.lastValidBlockHeight = block.lastValidBlockHeight
    txn.feePayer = owner
    txn.recentBlockhash = block.blockhash
    console.log('51 txn', { txn })
    for (const i of i_array) {
      txn.add(i)
    }
    transactions.push(txn)
  }
  console.log('build_transaction done')
  return transactions
}