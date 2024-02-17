import { ApiPromise } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { ContractCallOutcome, ContractOptions } from '@polkadot/api-contract/types';
import { EventRecord } from '@polkadot/types/interfaces';
import { Callback, IKeyringPair, ISubmittableResult } from '@polkadot/types/types';
import { BN, stringCamelCase } from '@polkadot/util';
import { decodeOutput, getAbiMessage } from './decodeOutput';
import { getExtrinsicErrorMessage } from './getExtrinsicErrorMessage';
import { getMaxGasLimit } from './getGasLimit';
import { Signer } from '@polkadot/api/types';


export const contractCallDryRun = async (
  api: ApiPromise,
  account: IKeyringPair | string,
  contract: ContractPromise,
  method: string,
  options = {} as ContractOptions,
  args = [] as unknown[],
): Promise<ContractCallOutcome> => {
  const abiMessage = getAbiMessage(contract, method)
  const address = (account as IKeyringPair)?.address || account
  const { value, gasLimit, storageDepositLimit } = options
  const result = await api.call.contractsApi.call<ContractCallOutcome>(
    address,
    contract.address,
    value ?? new BN(0),
    gasLimit ?? null,
    storageDepositLimit ?? null,
    abiMessage.toU8a(args),
  )

  return result
}

export type ContractTxResult = {
    dryResult: ContractCallOutcome
    result?: ISubmittableResult
    errorMessage?: ReturnType<typeof getExtrinsicErrorMessage> | 'ExtrinsicFailed'
    errorEvent?: EventRecord
    successEvent?: EventRecord
    extrinsicHash?: string
    extrinsicIndex?: number
    blockHash?: string
  }

export const contractTx = async (
  api: ApiPromise,
  account: IKeyringPair | string,
  signer: Signer | undefined,
  contract: ContractPromise,
  method: string,
  options = {} as ContractOptions,
  args = [] as unknown[],
  statusCb?: Callback<ISubmittableResult>,
): Promise<ContractTxResult> => {
  // Dry run to determine required gas and potential errors
  delete options.gasLimit
  const dryResult = await contractCallDryRun(api, account, contract, method, options, args)
  const { isError, decodedOutput } = decodeOutput(dryResult, contract, method)
  if (isError)
    return Promise.reject({
      dryResult,
      errorMessage: decodedOutput || 'Error',
    })

  // Call actual query/tx & wrap it in a promise
  const gasLimit = dryResult.gasRequired
  return new Promise(async (resolve, reject) => {
    try {
      const isDevelopment =
        (api.runtimeChain || '').toLowerCase() === 'development' ? 'isInBlock' : 'isFinalized'
      const finalStatus = isDevelopment ? 'isInBlock' : 'isFinalized'
      const asFinalStatus = isDevelopment ? 'asInBlock' : 'asFinalized'

      const tx = contract.tx[stringCamelCase(method)]({ ...options, gasLimit }, ...args)

      const unsub = await tx.signAndSend(account, { signer }, async (result) => {
        statusCb?.(result)

        const isFinalized = result?.status?.[finalStatus]
        if (!isFinalized) return

        // Determine extrinsic and block info
        const extrinsicHash = result.txHash.toHex()
        const extrinsicIndex = result.txIndex
        const blockHash = result.status[asFinalStatus].toHex()

        const errorEvent = result?.events.find(({ event }) =>
          api.events.system.ExtrinsicFailed.is(event),
        )
        if (errorEvent) {
          // Reject if `ExtrinsicFailed` event was found
          reject({
            dryResult,
            errorMessage: decodeOutput || 'ExtrinsicFailed',
            errorEvent,
            extrinsicHash,
            extrinsicIndex,
            blockHash,
          })
          unsub?.()
        } else {
          // Resolve succesfully otherwise
          const successEvent = result?.events.find(({ event }) =>
            api.events.system.ExtrinsicSuccess.is(event),
          )

          resolve({
            dryResult,
            result,
            successEvent,
            extrinsicHash,
            extrinsicIndex,
            blockHash,
          })
          unsub?.()
        }
      })
    } catch (e: any) {
      console.error('Error during contract transaction:', e)
      reject({ errorMessage: getExtrinsicErrorMessage(e), errorEvent: e })
    }
  })
}

export const contractQuery = async (
  api: ApiPromise,
  address: string,
  contract: ContractPromise,
  method: string,
  options = {} as ContractOptions,
  args = [] as unknown[],
): Promise<ContractCallOutcome> => {
  // HACK: This should be possible by setting the `gasLimit` to null or -1 in the future.
  const gasLimit = getMaxGasLimit(api)

  // Call actual query/tx
  const queryFn = contract.query[stringCamelCase(method)]
  return await queryFn(address, { ...options, gasLimit }, ...args)
}
