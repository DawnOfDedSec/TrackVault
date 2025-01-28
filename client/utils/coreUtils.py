def isDictEx(arr, key, v=None):
    cur = arr
    keys = key.split(".") if isinstance(key, str) else key
    try:
        for k in keys:
            if k not in cur:
                cur[k] = {}
            cur = cur[k]

        if v and cur == v:
            return True
        elif not v and cur:
            return True
        else:
            return False

    except Exception:
        return None
