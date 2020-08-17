from mipha.profiler import track


def test_profiler():
    with track("test.json"):
        print("test.json")
