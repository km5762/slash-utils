from tinyec import registry
import secrets

curve = registry.get_curve('secp521r1')

alicePrivKey = 0x1b30b938416c554ae08fbf2c85e61e25315c101241e4814794d318055717d98954ce50304198cb3e2d201f6c970c4185e956de90d659511507d90c2c4f009bbf138
alicePubKey = alicePrivKey * curve.g
print("Alice public key:", alicePubKey)

bobPrivKey = 0x21e0c9d3a50a5e7ffa8866e62c871a9fd8835d6e3df78dda5d761cd467edbf3601e81daacb309aaee06ce8886d6e1f373f5d66bb4ae872dcd4d24f84e1a8abe893
bobPubKey = bobPrivKey * curve.g
print("Bob public key:", bobPubKey)

print("Now exchange the public keys (e.g. through Internet)")

aliceSharedKey = alicePrivKey * bobPubKey
print("Alice shared key:", aliceSharedKey)

bobSharedKey = bobPrivKey * alicePubKey
print("Bob shared key:", bobSharedKey)

# Extract the x-coordinate of the shared key and print it in hex
aliceSharedKeyHex = hex(aliceSharedKey.x)[2:]  # Skip the '0x' prefix
print("Alice shared key in hex:", aliceSharedKeyHex)

bobSharedKeyHex = hex(bobSharedKey.x)[2:]  # Skip the '0x' prefix
print("Bob shared key in hex:", bobSharedKeyHex)