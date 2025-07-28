export interface ShapContribution {
    feature: string;
    impact: number;
  }
  
  export interface ExplanationPayload {
    event_id: string;
    shap_reasoning: {
      top_contributors: ShapContribution[];
    };
    trust_score_reasoning: {
      delta: number;
      reasons: string[];
    };
    gnn_context: {
      node: string;
      causal_parents: string[];
      related_nodes: string[];
    };
  }
  
