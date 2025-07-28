import json

def load_op_cleaver_static_features(file_path: str):
    with open(file_path, "r") as f:
        data = json.load(f)
    return data

if __name__ == "__main__":
    features = load_op_cleaver_static_features(
        "backend/llm_pipeline/extra_layer/data_collection/op_cleaver_static_features.json"
    )
    print("🔍 Loaded entries:", len(features))
    print("🧬 Sample entry:", features[0])
