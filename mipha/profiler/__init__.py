import contextlib
import sys
import time

from .speedscope import SpRecorder  # noqa


class Recorder:
    sp_recorder: SpRecorder

    def get_nanos(self):
        return int(time.perf_counter() * 1e9)

    def start(self):
        self._begin_time = self.get_nanos()
        self.sp_recorder = SpRecorder()
        sys.setprofile(self._trace_func)

    def stop(self):
        sys.setprofile(None)
        self._end_time = self.get_nanos()

    def export_to_json(self, filename):
        return self.sp_recorder.export_to_json(filename)

    def _trace_func(self, frame, event, arg):
        filename = frame.f_code.co_filename
        if filename == __file__:
            # Ignore this file itself
            return

        if event == "call":
            typ = "O"
        elif event == "return":
            typ = "C"
        else:
            # Ignore everything else.
            return

        timestamp = self.get_nanos()
        line = frame.f_code.co_firstlineno
        name = frame.f_code.co_name
        self.sp_recorder.append_record(timestamp, typ, filename, line, name)

    def length(self):
        return self.sp_recorder.len()


@contextlib.contextmanager
def track(filename):
    record = Recorder()
    record.start()
    sys.setprofile(record._trace_func)
    yield
    record.stop()
    t = record.export_to_json(filename)
    print("record export to json", t)
    assert False


# __all__ = ["profiler_start", "profiler_stop", "profiler_export"]
