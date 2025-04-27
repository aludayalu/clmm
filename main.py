from contract import execute, Keypair, scval
import random

contract = "CBLOG7JOUJORQN675KLSPB7OESGY5NKP2IUBKPMIXUKTAS3AIYJ4GG3E"

kp = Keypair.from_mnemonic_phrase("rural clay symptom young alone ticket pole any path awake crime false cement fish rather hood mom cabbage parent arrow toast setup jar front")

print(execute("get_balance", kp, contract, [scval.to_address(kp.public_key)], simulate=True))

exit()
print(execute("init", kp, contract, [scval.to_address(kp.public_key), scval.to_uint128(1000000)]))

position_id = random.randint(0, 1024 ** 4)

print(execute("open_position", kp, contract, [scval.to_uint128(5), scval.to_uint32(2), scval.to_address(kp.public_key), scval.to_uint128(position_id)]))