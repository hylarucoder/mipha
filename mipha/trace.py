from mipha.spy import Tracer  # noqa


def track():
    record = Tracer()
    r = record.trace(
        pid=88708,
        sample_rate=1,
        trace_line="toolbar/apps/admin.py",
    )
    print(r)
