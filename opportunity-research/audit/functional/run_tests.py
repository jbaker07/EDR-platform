"""Functional tests of what a tool-enabled assistant (this environment: Python with numpy/trimesh/Pillow, Node) can do
on representative inputs, in one pass, for the workbench families. Each test records: input, steps, result, caveats.
Outputs go to audit/functional/results.json and the artifacts next to it."""
import json, time, io, os, subprocess, struct
import numpy as np
from PIL import Image
import trimesh
OUT = os.path.dirname(os.path.abspath(__file__))
R = {}

# ---- Test 1: STL printability checks on a deliberately broken mesh (hole + flipped face + thin wall) ----
t0 = time.time()
box = trimesh.creation.box(extents=(20, 20, 20))
faces = box.faces.copy()
faces = np.delete(faces, 0, axis=0)            # open a hole
faces[1] = faces[1][::-1]                      # flip one face
broken = trimesh.Trimesh(vertices=box.vertices, faces=faces, process=False)
thin = trimesh.creation.box(extents=(20, 20, 0.3))   # 0.3 mm wall, below a typical 0.8 mm minimum
path = os.path.join(OUT, "broken_box.stl"); broken.export(path)
m = trimesh.load(path, force="mesh", process=False)
report = {
    "file": "broken_box.stl", "triangles": int(len(m.faces)), "watertight": bool(m.is_watertight),
    "winding_consistent": bool(m.is_winding_consistent), "volume_mm3": float(m.volume) if m.is_volume else None,
    "bounds_mm": m.bounds.tolist(), "euler_number": int(m.euler_number),
}
# repair attempt
rep = m.copy(); trimesh.repair.fix_normals(rep); trimesh.repair.fill_holes(rep)
report["after_repair_watertight"] = bool(rep.is_watertight)
report["after_repair_winding_consistent"] = bool(rep.is_winding_consistent)
# thin wall: approximate by ray-cast thickness on face centers
t = thin.copy()
thick = trimesh.proximity.thickness(t, t.triangles_center[:50], exterior=False, method="ray")
report["thin_plate_min_thickness_mm"] = float(np.nanmin(thick))
report["units_note"] = "STL carries no units; the file is assumed mm, the check reports bounds only"
report["overhang_check"] = "not attempted (needs orientation and printer profile)"
R["stl_checks"] = {"steps": ["load", "watertight/winding/volume", "fix_normals + fill_holes", "ray thickness"],
                   "seconds": round(time.time() - t0, 2), "result": report,
                   "caveat": "fill_holes closes simple holes only; self-intersections and non-manifold edges need a proper repair engine; this is basic mesh repair, not universal printability"}

# ---- Test 2: channel packing (R=metallic, G=roughness, B=AO) and normal-map green-channel inversion ----
t0 = time.time()
w = h = 256
metal = Image.fromarray((np.linspace(0, 255, w)[None, :].repeat(h, 0)).astype(np.uint8))
rough = Image.fromarray((np.linspace(255, 0, h)[:, None].repeat(w, 1)).astype(np.uint8))
ao = Image.fromarray(np.full((h, w), 200, np.uint8))
packed = Image.merge("RGB", (metal, rough, ao)); packed.save(os.path.join(OUT, "packed_orm.png"))
# normal map: synthesize, then invert green (DirectX <-> OpenGL)
nrm = np.zeros((h, w, 3), np.uint8); nrm[..., 0] = 128; nrm[..., 1] = 100; nrm[..., 2] = 255
n = Image.fromarray(nrm); r_, g_, b_ = n.split(); g_inv = g_.point(lambda v: 255 - v)
Image.merge("RGB", (r_, g_inv, b_)).save(os.path.join(OUT, "normal_gl.png"))
chk = Image.open(os.path.join(OUT, "packed_orm.png")).split()
R["channel_packing"] = {"steps": ["load three maps", "merge to RGB", "save; split to verify", "invert G for convention"],
                        "seconds": round(time.time() - t0, 2),
                        "result": {"packed_channels_verified": [int(np.array(c).mean()) for c in chk], "green_inverted_mean": int(np.array(g_inv).mean())},
                        "caveat": "assumes same resolution and linear (non-sRGB) data; sRGB handling and 16-bit maps need explicit conversion"}

# ---- Test 3: sprite sheet slicing by grid with pivot metadata ----
t0 = time.time()
sheet = Image.new("RGBA", (256, 64), (0, 0, 0, 0))
for i in range(4):
    Image.Image.paste(sheet, Image.new("RGBA", (40, 40), (255, 0, 0, 255)), (i * 64 + 12, 12))
sheet.save(os.path.join(OUT, "sheet.png"))
frames = []
for i in range(4):
    fr = sheet.crop((i * 64, 0, (i + 1) * 64, 64)); bbox = fr.getbbox()
    frames.append({"frame": i, "rect": [i * 64, 0, 64, 64], "trimmed_bbox": bbox, "pivot": [0.5, 1.0]})
json.dump({"image": "sheet.png", "frames": frames}, open(os.path.join(OUT, "sheet.json"), "w"), indent=1)
R["sprite_slicing"] = {"steps": ["load sheet", "grid crop", "trim bbox", "write JSON with pivots"], "seconds": round(time.time() - t0, 2),
                       "result": {"frames": len(frames), "trimmed_bbox_frame0": frames[0]["trimmed_bbox"]},
                       "caveat": "grid slicing only; irregular sheets need connected-component detection; engine-specific formats (Unity .meta, Godot .tres) not produced"}

# ---- Test 4: glTF inspection with glTF Transform CLI (npx) on a generated GLB ----
t0 = time.time()
glb = os.path.join(OUT, "box.glb"); trimesh.creation.box(extents=(1, 2, 3)).export(glb)
try:
    p = subprocess.run(["npx", "-y", "@gltf-transform/cli", "inspect", glb], capture_output=True, text=True, timeout=300)
    out = (p.stdout + p.stderr)[-1500:]
    ok = p.returncode == 0
except Exception as e:
    out, ok = str(e), False
R["gltf_inspect"] = {"steps": ["export GLB", "npx @gltf-transform/cli inspect"], "seconds": round(time.time() - t0, 2),
                     "result": {"ok": ok, "output_tail": out}, "caveat": "requires Node and a network install of the CLI; the same report is what gltf.report shows in a browser"}

json.dump(R, open(os.path.join(OUT, "results.json"), "w"), indent=1, default=str)
print(json.dumps({k: {"seconds": v["seconds"], "ok": v["result"]} for k, v in R.items()}, indent=1, default=str)[:3000])
