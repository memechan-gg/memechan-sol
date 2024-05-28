import base64
import struct
import base58

# JSON data to encode
parsed_data = {
        "name": "addressLookupTable",
        "data": {
            "lookupTableMeta": {
                "deactivationSlot": "18446744073709551615",
                "lastExtendedSlot": "209620855",
                "lastExtendedSlotStartIndex": 0,
                "authority": "RayZuc5vEK174xfgNFdD9YADqbbwbFjVjY4NM8itSF9",
                "padding": 0
            },
            "lookupTableAddresses": {
                "addresses": [
                    "11111111111111111111111111111111",
                    "ComputeBudget111111111111111111111111111111",
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                    "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo",
                    "SysvarRent111111111111111111111111111111111",
                    "SysvarC1ock11111111111111111111111111111111",
                    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
                    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
                    "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o",
                    "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
                    "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
                    "RVKd61ztZW9GUwhRbbLoYVRE5Xf1B2tVscKqwZqXgEr",
                    "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv",
                    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                    "5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h",
                    "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
                    "routeUGWgWzqBWFcrCfv8tritsqukccJPu3q5GPP3xS",
                    "EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q",
                    "CBuCnLe26faBpcBP2fktp4rp8abpcAnTWft6ZrP5Q4T",
                    "9KEPoZmtHUrBbhWN1v1KWLMkkvwY6WLtAVUCPRtRjP4z",
                    "6FJon3QE27qgPVggARueB22hLvoh22VzJpXv4rBEoSLF",
                    "CC12se5To1CdEuw7fDS27B7Geo5jJyL7t5UK2B44NgiH",
                    "9HzJyW1qZsEiSfMUf6L2jo3CcTKAyBmSyKdwQeYisHrC"
                ]
            },
            "type": "account"
        },
    "program": "address_lookup_table",
    "space": 824
}

def encode_address_lookup_table(parsed_data):
    # Extract lookupTableMeta
    deactivation_slot = int(parsed_data["data"]["lookupTableMeta"]["deactivationSlot"])
    last_extended_slot = int(parsed_data["data"]["lookupTableMeta"]["lastExtendedSlot"])
    last_extended_slot_start_index = parsed_data["data"]["lookupTableMeta"]["lastExtendedSlotStartIndex"]
    authority = parsed_data["data"]["lookupTableMeta"]["authority"].encode('utf-8')
    padding = int(parsed_data["data"]["lookupTableMeta"]["padding"])

    # Pack the lookupTableMeta
    lookup_table_meta = struct.pack('<QIQ', deactivation_slot, last_extended_slot, last_extended_slot_start_index) + authority + struct.pack('<Q', padding)

    # Extract lookupTableAddresses
    addresses = parsed_data["data"]["lookupTableAddresses"]["addresses"]
    address_length = 32  # Each address is 32 bytes

    # Pack the lookupTableAddresses
    lookup_table_addresses = b''
    for address in addresses:
        decoded_address = base58.b58decode(address)
        lookup_table_addresses += decoded_address

    # Combine the packed data
    encoded_data = lookup_table_meta + lookup_table_addresses

    # Base64 encode the result
    base64_encoded_data = base64.b64encode(encoded_data)

    # Ensure padding for base64
    padding_length = len(encoded_data) % 4
    if padding_length > 0:
        base64_encoded_data += b'=' * (4 - padding_length)

    return base64_encoded_data.decode('utf-8')

# Encode the provided data
encoded_data = encode_address_lookup_table(parsed_data)
print(encoded_data)