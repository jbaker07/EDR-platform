"""Atlas extraction utilities.

Everything under ``atlas/extracted/`` is produced by these modules from the
artifacts the project actually resolved, and carries the sha256 of the artifact
it was read from. Nothing here is authored by hand; if a fact is wrong, the fix
is to the extractor or the artifact pin, never to the JSON.
"""
