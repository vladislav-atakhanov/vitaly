import os
import sys
import json

def load(path, file):
    with open(f"{path}/{file}") as fd:
        data = json.loads(fd.read())
    return data['vendorId'].lower(), data['productId'].lower()

cut = 'keyboards/src'
cut_len = sys.argv[1].index(cut) + len(cut) + 1
refs = {}

for (path, dirs, files) in os.walk(sys.argv[1]):
    for file in files:
        if file.endswith('.json'):
            vendor_id, product_id = load(path, file)
            refs[f"{vendor_id}_{product_id}"] = f"{path[cut_len:]}/{file}"

print(json.dumps(refs, indent=4))
