import os
import sys
import json

def load(path, file):
    with open(f"{path}/{file}") as fd:
        data = json.loads(fd.read())
    return int(data['vendorId'].lower(), 16), int(data['productId'].lower(), 16)

cut = 'keyboards/src'
cut_len = sys.argv[1].index(cut) + len(cut) + 1
refs = {}

for (path, dirs, files) in os.walk(sys.argv[1]):
    for file in files:
        if file.endswith('.json'):
            vendor_id, product_id = load(path, file)
            refs[f"{vendor_id:#06x}_{product_id:#06x}"] = f"{path[cut_len:]}/{file}"

print(json.dumps(refs, indent=4))
