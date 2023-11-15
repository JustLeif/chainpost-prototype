const functions = require("@google-cloud/functions-framework")
const { Buffer } = require("buffer")
const { COSESign1, COSEKey, BigNum, Label, Int } = require("@emurgo/cardano-message-signing-nodejs")
const { Ed25519Signature, RewardAddress, PublicKey, Address } = require("@emurgo/cardano-serialization-lib-nodejs")

functions.http("cardano-validate-signature", validateSignature)

async function validateSignature(req, res) {
    console.log(req.body)
    const bodySignature = req.body.signature
    const bodyKey = req.body.key
    const result = validateCardanoDataSignature(bodySignature, bodyKey)
    res.status(200).send(result)
}

/**
 * Copied from the Cardano documentation to accurately validate the signature. (https://developers.cardano.org/docs/integrate-cardano/user-wallet-authentication/) 
 * @param {string} bodySignature 
 * @param {string} bodyKey 
 * @returns
 */
function validateCardanoDataSignature(bodySignature, bodyKey) {
    const decoded = COSESign1.from_bytes(Buffer.from(bodySignature, "hex"))
    console.log(decoded)
    const headermap = decoded.headers().protected().deserialized_headers()
    const addressHex = Buffer.from(headermap.header(Label.new_text("address")).to_bytes())
        .toString("hex")
        .substring(4)
    const address = Address.from_bytes(Buffer.from(addressHex, "hex"))
    const key = COSEKey.from_bytes(Buffer.from(bodyKey, "hex"))
    const pubKeyBytes = key.header(Label.new_int(Int.new_negative(BigNum.from_str("2")))).as_bytes()
    const publicKey = PublicKey.from_bytes(pubKeyBytes)
    const payload = decoded.payload()
    const signature = Ed25519Signature.from_bytes(decoded.signature())
    const receivedData = decoded.signed_data().to_bytes()
    const signerStakeAddrBech32 = RewardAddress.from_address(address).to_address().to_bech32()
    const utf8Payload = Buffer.from(payload).toString("utf8")
    const expectedPayload = `chainpost_auth ${signerStakeAddrBech32}`
    const isVerified = publicKey.verify(receivedData, signature)
    const payloadAsExpected = utf8Payload == expectedPayload
    const result = {
        /**
         * @type {string}
         */
        uid: signerStakeAddrBech32,
        /**
         * @type {boolean}
         */
        validSignature: isVerified && payloadAsExpected
    }
    return result
}