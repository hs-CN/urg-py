import urg_py

urg = urg_py.Urg("192.168.0.10", 10940)

print(urg.get_version_info())
print(urg.get_sensor_params())
print(urg.get_status_info())

urg.start_capture()

print(urg.get_distance(0, 1080, 0))
print(urg.get_distance_intensity(0, 1080, 0))

iter = urg.get_distance_multi(0, 1080, 0, 0, 10)
for i, payload in enumerate(iter):
    print(f"count: {i}")
    print(payload)

iter = urg.get_distance_intensity_multi(0, 1080, 0, 0, 10)
for i, payload in enumerate(iter):
    print(f"count: {i}")
    print(payload)

urg.stop_capture()
