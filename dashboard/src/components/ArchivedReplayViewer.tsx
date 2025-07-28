import React, { useEffect, useState } from 'react';
import GNNReplay from './GNNReplay';

interface ArchiveMeta {
  filename: string;
  incident_id: string;
  tags: string[];
  timestamp: string;
}

const ArchivedReplayViewer: React.FC = () => {
  const [archives, setArchives] = useState<ArchiveMeta[]>([]);
  const [filteredArchives, setFilteredArchives] = useState<ArchiveMeta[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [entryNode, setEntryNode] = useState<string | null>(null);
  const [activeTag, setActiveTag] = useState<string>("");

  useEffect(() => {
    fetch('/api/gnn/archive/list')
      .then(res => res.json())
      .then(data => {
        setArchives(data.archives || []);
        setFilteredArchives(data.archives || []);
      });
  }, []);

  const loadArchive = (filename: string) => {
    fetch(`/api/gnn/archive/load?filename=${filename}`)
      .then(res => res.json())
      .then(data => {
        setEntryNode(data.nodes[0] || null); // Assume first node is the origin
      });
    setSelectedFile(filename);
  };

  const handleTagFilter = (tag: string) => {
    setActiveTag(tag);
    if (tag === "") {
      setFilteredArchives(archives);
    } else {
      setFilteredArchives(archives.filter(a => a.tags.includes(tag)));
    }
  };

  const allTags = Array.from(new Set(archives.flatMap(a => a.tags)));

  return (
    <div className="mt-8 p-4 bg-white shadow rounded">
      <h2 className="text-xl font-semibold mb-4">📦 Archived GNN Replay Viewer</h2>

      <div className="mb-4 grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-1">Filter by Tag:</label>
          <select
            onChange={(e) => handleTagFilter(e.target.value)}
            className="border px-3 py-2 rounded w-full"
            value={activeTag}
          >
            <option value="">All</option>
            {allTags.map((tag, i) => (
              <option key={i} value={tag}>{tag}</option>
            ))}
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Select Archive:</label>
          <select
            onChange={(e) => loadArchive(e.target.value)}
            className="border px-3 py-2 rounded w-full"
            value={selectedFile || ''}
          >
            <option value="" disabled>Select snapshot...</option>
            {filteredArchives.map((archive, i) => (
              <option key={i} value={archive.filename}>
                {archive.incident_id} – {archive.timestamp}
              </option>
            ))}
          </select>
        </div>
      </div>

      {entryNode && (
        <div className="mt-6">
          <GNNReplay eventId={entryNode} />
        </div>
      )}
    </div>
  );
};

export default ArchivedReplayViewer;
