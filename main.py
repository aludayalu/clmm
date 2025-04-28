from contract import execute, Keypair, scval
import random

contract = "CCZ6QLFLTWEPSDHCVFDHRKR5S3MA56BP4Y3DCTJD6N6DM24ZWEBK4TXY"

kp = Keypair.from_mnemonic_phrase("rural clay symptom young alone ticket pole any path awake crime false cement fish rather hood mom cabbage parent arrow toast setup jar front")

# print(execute("init", kp, contract, [scval.to_address(kp.public_key), scval.to_uint128(1000000)]))

position_id = random.randint(0, 1024 ** 4)
# print(execute("open_position", kp, contract, [scval.to_uint128(5000), scval.to_uint32(1), scval.to_address(kp.public_key), scval.to_uint128(position_id)]))
# exit()
print(execute("get_balance", kp, contract, [scval.to_address(kp.public_key)], simulate=True))

print(execute("swap", kp, contract, [scval.to_uint128(100), scval.to_bool(False), scval.to_address(kp.public_key)]))

print(execute("get_balance", kp, contract, [scval.to_address(kp.public_key)], simulate=True))