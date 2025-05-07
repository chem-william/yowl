import json

with open("all_smiles", "r") as fd:
    data = json.load(fd)

with open("parsed_smiles.smi", "w") as fd:
    for item in data:
        fd.write(item["smiles"] + "\n")
