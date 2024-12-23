import { FC, useState } from 'react'
import { withMainLayout } from '../guards/LayoutGuard'
import { withWalletGuard } from '../guards/WalletGuard'
import { Checkbox, Spinner, Table } from "flowbite-react";
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { useMetaplex } from '../contexts/MetaplexContext';
import { usePyth } from '../contexts/PythContext';
import { get_all_token_accounts, TokenAccountDetails } from '../utils/get_all_token_accounts';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { Button } from "flowbite-react";
import { CloseAccountInstructionInput, build_transaction } from '../utils/close_account_instruction';
import { Pagination } from "flowbite-react";
import { Card } from "flowbite-react";

const Home: FC = () => {
  const { connection } = useConnection()
  const { publicKey, signAllTransactions, sendTransaction } = useWallet()
  const { metaplex } = useMetaplex()
  const { priceUsd } = usePyth()
  const [rows, setRows] = useState<TokenAccountDetails[]>([])
  const [selected, _setSelected] = useState<{ [key: string]: TokenAccountDetails | undefined }>({})
  const [loading, setLoading] = useState<boolean | undefined>(undefined)
  const [totalUsd, setTotalUsd] = useState(0)
  const [totalSol, setTotalSol] = useState(0)
  const [allChecked, setAllChecked] = useState(false);

  const [totalPages, setTotalPages] = useState(100);
  const [currentPage, setCurrentPage] = useState(1);
  const onPageChange = (page: number) => setCurrentPage(page);


  async function get_token_accounts() {
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
      return
    }
    setLoading(true)
    const token_accounts = await get_all_token_accounts(connection, publicKey, metaplex)
    console.log('token_accounts', { token_accounts })
    setRows(token_accounts)
    setTotalPages(Math.ceil(token_accounts.length / 10));
    setLoading(false)
  }

  async function lfg() {
    if (!sendTransaction) {
      console.error('Connect wallet')
      return
    }
    if (!signAllTransactions) {
      console.error('Connect wallet')
      return
    }
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
      return
    }
    if (!connection) {
      console.error('No connection')
      return
    }

    const inputs: CloseAccountInstructionInput[] = []
    for (const entry of Object.entries(selected)) {
      const [_mint, details] = entry
      if (details === undefined) {
        continue
      }
      const input: CloseAccountInstructionInput = {
        account: new PublicKey(details.address),
        destination: publicKey,
        authority: publicKey
      }
      inputs.push(input)
    }

    const txns = await build_transaction(connection, publicKey, inputs)
    console.log('sending it', txns)
    const signedTxns = await signAllTransactions(txns)
    console.log('signedTxns', signedTxns)
    for (const txn of signedTxns) {
      console.log('txn', txn)
      await sendTransaction(txn, connection)
    }
  }

  function selectMint(i: TokenAccountDetails, mint: string, usd: number, sol: number) {
    if (selected[mint]) {
      selected[mint] = undefined
      setTotalSol((value) => value - sol)
      setTotalUsd((value) => value - usd)
    } else {
      selected[mint] = i
      setTotalSol((value) => value + sol)
      setTotalUsd((value) => value + usd)
    }
  }

  function handleAllChecked() {
    setTotalSol(0);
    setTotalUsd(0);
    if (allChecked) {
      rows.forEach((row) => {
        selected[row.mint] = undefined;
      })
    } else {
      rows.forEach((row, idx) => {
        setTotalSol((value) => value + row.lamports / LAMPORTS_PER_SOL)
        setTotalUsd((value) => value + (priceUsd || 0) * row.lamports / LAMPORTS_PER_SOL)
        selected[row.mint] = rows[idx];
      })
    }
    setAllChecked(!allChecked)
  }
  
  return (
    <>
        <div >
          <div className="px-40 my-10 flex justify-between items-center">
            {
              loading ? 
              <Button>
                <Spinner aria-label="Spinner button example" size="sm" />
                <span className="pl-3">Get Token Accounts...</span>
              </Button> : <Button color="dark" onClick={get_token_accounts}> Get Token Accounts </Button>
            }
            <Button color="dark" onClick={lfg}> Recycle Accounts </Button>
            <p><strong>Total $USD:</strong><span className={'m-1'}>{totalUsd.toFixed(3)}</span></p>
            <p><strong>Total $SOL:</strong><span className={'m-1'}>{totalSol.toFixed(3)}</span></p>
          </div>
          {
           loading == undefined || (loading != undefined && rows.length != 0) || loading ? 
           <div>
            <div className='overflow-x-auto my-4 mx-20'> 
              <Table hoverable>
                <Table.Head>
                  <Table.HeadCell className="p-4">
                    <Checkbox checked={allChecked} onClick={handleAllChecked}/>
                  </Table.HeadCell>
                  <Table.HeadCell>Name</Table.HeadCell>
                  <Table.HeadCell>Mint</Table.HeadCell>
                  <Table.HeadCell>Amount</Table.HeadCell>
                  <Table.HeadCell>$SOL</Table.HeadCell>
                  <Table.HeadCell>$USD</Table.HeadCell>
                </Table.Head>
                <Table.Body className="divide-y">
                {
                rows.slice((currentPage - 1) * 10, currentPage * 10 > rows.length ? rows.length : currentPage * 10).map((row, idx) =>
                  <Table.Row className="bg-white dark:border-gray-700 dark:bg-gray-800" key={idx}>
                    <Table.Cell className="p-4">
                      <Checkbox checked={selected[row.mint] !== undefined}
                        onClick={() => selectMint(row, row.mint, (priceUsd || 0) * row.lamports / LAMPORTS_PER_SOL, row.lamports / LAMPORTS_PER_SOL)}/>
                    </Table.Cell>
                    <Table.Cell className="whitespace-nowrap font-medium text-gray-900 dark:text-white">
                      {row.name}
                    </Table.Cell>
                    <Table.Cell>
                      <a href={`https://explorer.solana.com/address/${row.mint}`} target={'_blank'}>
                        {row.mint}
                      </a>
                    </Table.Cell>
                    <Table.Cell>
                      {row.amount}
                    </Table.Cell>
                    <Table.Cell>
                      {(row.lamports / LAMPORTS_PER_SOL).toFixed(3)}
                    </Table.Cell>
                    <Table.Cell>
                      {((priceUsd || 0) * (row.lamports / LAMPORTS_PER_SOL)).toFixed(3)}
                    </Table.Cell>
                  </Table.Row>
                )
              }
                </Table.Body>
              </Table>
            </div>
            <div className="flex overflow-x-auto sm:justify-center">
              <Pagination
                layout="navigation"
                currentPage={currentPage}
                totalPages={totalPages}
                onPageChange={onPageChange}
                showIcons
              />
            </div>
          </div> : 
          <div className='flex justify-center'>
          <Card href="#" className="max-w-xl">
            <h5 className="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
              Ooops! Seems you have no token accounts!
            </h5>
          </Card>
        </div>
          }
        </div>
    </>
  )
}

export default withMainLayout(withWalletGuard(Home))