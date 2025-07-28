
def get_dynamic_threshold(entity_profile, default_threshold=0.5):
    role = entity_profile.get("role", "generic")
    hour = entity_profile.get("hour", 12)
    recent_flags = entity_profile.get("recent_flags", 0)

    threshold = default_threshold

    if role in ["admin", "devops", "domain_admin"]:
        threshold -= 0.1
    if hour < 6 or hour > 20:
        threshold -= 0.05
    if recent_flags >= 3:
        threshold -= 0.1

    return max(0.1, min(threshold, 1.0))
