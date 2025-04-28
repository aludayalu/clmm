from stellar_sdk import Keypair
from contract import execute, scval

contract = "CC4L2YWTQIBHKQ55ITRGFFWONV6J5UR4JGVA6ARFG4WRZMMIQ3QAXWOU"
kp = Keypair.from_mnemonic_phrase("rural clay symptom young alone ticket pole any path awake crime false cement fish rather hood mom cabbage parent arrow toast setup jar front")

def sign_payload(amount: int, secret_seed: str) -> tuple[bytes, bytes, bytes]:
    prefix = 69696969
    payload = prefix.to_bytes(16, "big") + amount.to_bytes(16, "big")
    kp = Keypair.from_secret(secret_seed)
    signature = kp.sign(payload)
    return [payload, signature]

# execute("start_channel", kp, contract, [scval.to_address(kp.public_key), scval.to_address("GCK3J7CEYAVPT23LBXI2QS4XMWQB4OIFYGN3AFGGZJINQUXCIVCP7FKZ"), scval.to_uint128(1000), scval.to_uint128(1234)])

print(execute("get_channel", kp, contract, [scval.to_uint128(1234)], simulate=True))