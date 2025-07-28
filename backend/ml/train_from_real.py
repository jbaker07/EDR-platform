import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.neural_network import MLPClassifier
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.pipeline import Pipeline
from joblib import dump

# Load labeled telemetry
df = pd.read_csv("backend/telemetry_logs/labeled_telemetry.csv")

# Select features and target
features = [
    "cpu_percent", "memory", "pid"
    # Add more fields if needed and numeric
]
X = df[features]
y = df["label"]

# Encode labels (benign = 0, malicious = 1)
le = LabelEncoder()
y_encoded = le.fit_transform(y)

# Split data
X_train, X_test, y_train, y_test = train_test_split(X, y_encoded, test_size=0.2, random_state=42)

# Create pipeline
pipeline = Pipeline([
    ("scaler", StandardScaler()),
    ("clf", MLPClassifier(hidden_layer_sizes=(64, 32), max_iter=300, random_state=42))
])

# Train model
pipeline.fit(X_train, y_train)

# Save model
dump(pipeline, "backend/ml/real_telemetry_model.joblib")
print("✅ Trained and saved model → backend/ml/real_telemetry_model.joblib")
