import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.neural_network import MLPClassifier
from sklearn.metrics import classification_report
from sklearn.impute import SimpleImputer

# Load labeled data
path = "backend_2/llm_pipeline_2/data_2/processed_2/labeled_embeddings.csv"

df = pd.read_csv(path)

# Drop non-numeric columns if present
if "Filename" in df.columns:
    df = df.drop(columns=["Filename"])

# Separate features and target
X = df.drop(columns=["label"])
y = df["label"]

# Impute missing values (replace NaNs with mean)
imputer = SimpleImputer(strategy="mean")
X_imputed = imputer.fit_transform(X)

# Train/test split
X_train, X_test, y_train, y_test = train_test_split(X_imputed, y, test_size=0.2, random_state=42)

# Define and train the MLP model
clf = MLPClassifier(hidden_layer_sizes=(64, 32), max_iter=300, random_state=42)
clf.fit(X_train, y_train)

# Predict and evaluate
y_pred = clf.predict(X_test)
print(classification_report(y_test, y_pred))


import joblib

# Assuming your model is named `mlp`
joblib.dump(mlp, "backend/backend_2/llm_pipeline_2/models/mlp_model.pkl")

