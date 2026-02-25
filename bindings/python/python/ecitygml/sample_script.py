from typing import Optional
import time

from ecitygml import GmlReader
from ecitygml import Envelope

path = "../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml"

print("Reading CityGML file from " + path + " ...")
start_time = time.time()
reader = GmlReader(path)
city_model = reader.finish()
print("Finished reading CityGML file in %s seconds" % (time.time() - start_time))
city_model.refresh_bounded_by_recursive()

city_objects = city_model.city_objects()
print("Total number of objects: " + str(len(city_objects)))

for co in city_objects:
    envelope: Optional[Envelope] = co.bounded_by()
    print("ID: " + co.id() + "; class: " + str(co.city_object_class()), end="")
    if envelope is not None:
        print("; envelope: " + str(envelope.lower_corner) + " - " + str(envelope.upper_corner), end="")
    print("")
