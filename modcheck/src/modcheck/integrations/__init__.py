"""Integrations with existing, maintained upstream tooling and databases.

We do not rebuild what these already do well. Each integration reads an
upstream artifact we fetched (with provenance) and translates it into ModCheck
findings, preserving the upstream semantics rather than flattening them.
"""
