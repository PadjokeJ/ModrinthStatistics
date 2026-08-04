
def main() -> None:
    import json
    LOADERS = ["fabric", "forge", "neoforge", "quilt"]

    i = 0
    data = {
        "licenses": {},
        "versions": {},
        "loaders": {}
    }
    while True:
        res = {}
        try:
            with open(f"results/{i}.json", 'r', encoding="utf8") as f:
                res = json.load(f)

        except:
            with open(f"data.json", 'w', encoding="utf8") as f:
                json.dump(data, f, ensure_ascii=False)
            print("Ran out of files to parse")
            return
        
        for hit in res["hits"]:
            licenses = hit["license"]
            versions = hit["versions"]
            loaders = hit["categories"]

            for j in loaders:
                if j in LOADERS:
                    count = data["loaders"].get(j, 0) + 1
                    data["loaders"][j] = count

            for j in versions:
                count = data["versions"].get(j, 0) + 1
                data["versions"][j] = count

            count = data["licenses"].get(licenses, 0) + 1
            data["licenses"][licenses] = count

        i += 100

if __name__ == "__main__":
    main()

