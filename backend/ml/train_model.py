import pandas as pd
import joblib
from sklearn.ensemble import RandomForestClassifier

df = pd.read_csv("logs/vectorized_processes.csv")

# Basic labeling assumption for now
df["label"] = df["name"].apply(lambda x: 1 if "malware" in x.lower() else 0)

X = df[["memory", "cpu_usage"]]
y = df["label"]

clf = RandomForestClassifier(n_estimators=100, random_state=42)
clf.fit(X, y)

joblib.dump(clf, "models/process_classifier.pkl")
print("✅ Model trained and saved.")
