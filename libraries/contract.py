from stellar_sdk import Network, Keypair, TransactionBuilder, Server, SorobanServer, stellar_xdr, scval
from stellar_sdk.soroban_rpc import GetTransactionStatus, SendTransactionStatus
import time

def execute(function, kp, contract, args=[], simulate=False, printLogs=False):
    rpc = Server("http://localhost:8000")
    account = rpc.load_account(kp.public_key)
    tx = (
        TransactionBuilder(
            source_account=account,
            network_passphrase=Network.STANDALONE_NETWORK_PASSPHRASE,
            base_fee=rpc.fetch_base_fee()
        )
        .append_invoke_contract_function_op(
            contract_id=contract,
            function_name=function,
            parameters=args
        )
        .set_timeout(30)
        .build()
    )
    rps = SorobanServer("http://localhost:8000/soroban/rpc")
    if simulate:
        tx = rps.simulate_transaction(tx)
        if printLogs:
            for x in tx.events[2:]:
                print(stellar_xdr.DiagnosticEvent.from_xdr(x).event)
            return
        return stellar_xdr.SCVal.from_xdr(tx.results[0].xdr)
    try:
        tx = rps.prepare_transaction(tx)
    except Exception as e:
        print(e)
        print(e.simulate_transaction_response)
        exit()
    tx.sign(kp)
    res = rps.send_transaction(tx)
    if res.status != SendTransactionStatus.PENDING:
        raise Exception(res.error_result_xdr)
    while True:
        result = rps.get_transaction(res.hash)
        if result.status != GetTransactionStatus.NOT_FOUND:
            break
        time.sleep(1)
    if result.status == GetTransactionStatus.SUCCESS:
        transaction_meta_xdr = result.result_meta_xdr
    else:
        raise Exception(stellar_xdr.TransactionResult.from_xdr(result.result_xdr))
    tx_meta = stellar_xdr.TransactionMeta.from_xdr(result.result_meta_xdr)
    scval = tx_meta.v3.soroban_meta.return_value
    return scval