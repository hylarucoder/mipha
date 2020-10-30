import psutil

for proc in psutil.process_iter(['pid', 'name', 'username', "cmdline"]):
    segs = proc.info["cmdline"] or []
    for seg in segs:
        if "gunicorn" in seg:
            print(f"PID: {proc.pid}", seg)
            print(proc.parent().cmdline())
            break
