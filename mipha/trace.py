from .spy import Tracer  # noqa


def track():
    record = Tracer()
    record.trace(
        pid=3138,
        sample_rate=1,
        trace_line="toolbar/apps/admin.py",
    )
