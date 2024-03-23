# OP_RETURN-adder
 Add OP_RETURN to an existing transaction


USAGE: ./OpReturn_adder_testnet tx_hex "message to insert"


## Considerations

The transaction must created with sighash "SINGLE" or "SINGLE | ANYONECANPAY".

Have into account that when adding the OP_RETURN the fee rate will be a bit lower
