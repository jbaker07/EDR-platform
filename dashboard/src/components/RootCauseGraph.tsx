import React, { useEffect, useState } from 'react';
import ForceGraph2D from 'react-force-graph-2d';

interface Props {
  eventId: string;
}

interface GraphNode {
  id: string;
  label: string;
  color: string;
  x?: number;
  y?: number;
}

interface GraphLink {
  source: string;
  target: string;
}

interface ExplanationPayload {
  shap_reasoning: { top_contributors: { feature: string; impact: number }[] };
  trust_score_reasoning: { delta: number; reasons: string[] };
  gnn_context: { node: string; causal_parents: string[]; related_nodes: string[] };
}

const RootCauseGraph: React.FC<Props> = ({ eventId }) => {
  const [graphData, setGraphData] = useState<{ nodes: GraphNode[]; links: GraphLink[] }>({ nodes: [], links: [] });
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [explanation, setExplanation] = useState<ExplanationPayload | null>(null);

  useEffect(() => {
    fetch(`/api/explain/root_cause?event_id=${eventId}`)
      .then(res => res.json())
      .then(data => {
        const { compromised, origin = [], neighbors = [] } = data;
        const nodes: GraphNode[] = [
          { id: compromised, label: 'Compromised', color: 'red' },
          ...origin.map((p: string) => ({ id: p, label: 'Cause', color: 'orange' })),
          ...neighbors.map((n: string) => ({ id: n, label: 'Neighbor', color: 'gray' })),
        ];
        const links: GraphLink[] = [
          ...origin.map((p: string) => ({ source: p, target: compromised })),
          ...neighbors.map((n: string) => ({ source: compromised, target: n })),
        ];
        setGraphData({ nodes, links });
      });
  }, [eventId]);

  const handleNodeClick = (node: GraphNode) => {
    setSelectedNode(node.id);
    fetch(`/api/assistant/explain?event_id=${node.id}`)
      .then(res => res.json())
      .then(setExplanation)
      .catch(() => setExplanation(null));
  };

  return (
    <div className="mt-6 border-t pt-4">
      <h3 className="font-semibold text-gray-800 mb-2">Visual Root Cause Trace</h3>
      <div className="w-full h-[400px] bg-gray-100 rounded">
        <ForceGraph2D
          graphData={graphData}
          onNodeClick={handleNodeClick}
          nodeLabel="id"
          nodeAutoColorBy="label"
          nodeCanvasObject={(node: any, ctx: CanvasRenderingContext2D) => {
            if (typeof node.x !== 'number' || typeof node.y !== 'number') return;
            ctx.beginPath();
            ctx.arc(node.x, node.y, 5, 0, 2 * Math.PI);
            ctx.fillStyle = node.color || 'black';
            ctx.fill();
            ctx.font = '10px Sans-Serif';
            ctx.fillText(node.id, node.x + 6, node.y + 3);
          }}
        />
      </div>

      {explanation && selectedNode && (
        <div className="mt-4 text-sm text-gray-700 bg-white p-4 rounded shadow">
          <h4 className="font-semibold mb-1">Explainability for: {selectedNode}</h4>
          <div>
            <strong>SHAP Top Features:</strong>
            <ul className="list-disc ml-5">
              {explanation.shap_reasoning.top_contributors.map((f, i) => (
                <li key={i}>{f.feature}: {f.impact.toFixed(3)}</li>
              ))}
            </ul>
          </div>
          <div className="mt-2">
            <strong>Trust Impact:</strong> Δ{explanation.trust_score_reasoning.delta}
            <ul className="list-disc ml-5">
              {explanation.trust_score_reasoning.reasons.map((r, i) => <li key={i}>{r}</li>)}
            </ul>
          </div>
        </div>
      )}
    </div>
  );
};

export default RootCauseGraph;
